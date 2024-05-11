#!/usr/bin/python3

import subprocess
import time
import argparse


class TestData:
    def __init__(self, ip, country, city):
        self.ip = ip
        self.country = country
        self.city = city


# Fail gracefully if psutil package is not installed
try:
    import psutil
except ImportError:
    print("'psutil' not found, install with:\n> python3 -m pip install psutil")
    exit(1)


def ns_to_ms(ns):
    ns_to_ms = 1000000
    return ns / ns_to_ms


def b_to_mb(b):
    b_to_mb = 1024*1024
    return b / b_to_mb


def get_memory_usage(process):
    process = psutil.Process(process.pid)
    return process.memory_info().rss


def format_memory_usage(rss):
    KB = 1024
    MB = 1024 * 1024
    GB = 1024 * 1024 * 1024

    if rss >= GB:
        return f'{round(rss / GB, 2)}gb'
    elif rss >= MB:
        return f'{round(rss / MB, 2)}mb'
    elif rss >= KB:
        return f'{round(rss / KB, 2)}kb'
    else:
        return f'{round(rss, 2)}b'


def format_time(nanoseconds):
    if nanoseconds < 1000:
        return f'{int(nanoseconds)}ns'
    elif nanoseconds < 1000000:
        return f'{int(nanoseconds / 1000)}μs'
    elif nanoseconds < 1000000000:
        return f'{int(nanoseconds / 1000000)}ms'
    else:
        return f'{int(nanoseconds / 1000000000)}s'


def wait_ready(process):
    """Wait for process under test to indicate readyness"""
    resp = process.stdout.readline()
    if resp.strip() != "READY":
        raise Exception("The app failed to indicate readyness")


def send_command(process, command):
    """Send command to the process under test and measure processing time"""
    start = time.time_ns()
    process.stdin.write(command + "\n")
    result = process.stdout.readline()
    end = time.time_ns()
    duration = (end - start)

    memory_usage = get_memory_usage(process)

    return (result, duration, memory_usage)


def send_load_command(process):
    """Send command to the process under test to load the database"""
    result, time, memory_usage = send_command(process, "LOAD")
    if result.strip() != "OK":
            raise Exception("The app failed to load the database")
    return (time, memory_usage)


def send_exit_command(process):
    """Send exit command to the process under test"""
    send_command(process, "EXIT")


def send_lookup_command(process, ip):
    """Send command to the process under test to perform geolocation loookup"""
    result, time, memory_usage = send_command(process, "LOOKUP " + ip)

    tokens = result.strip().split(',')
    if len(tokens) != 2:
        raise Exception(f'Invalid response - "{result}"')

    return (tokens[0], tokens[1], time, memory_usage)


if __name__ == "__main__":
    # Parse CLI arguments
    parser = argparse.ArgumentParser(description="An utility implementing "
                                     "line-based request-response "
                                     "protocol for perfoming "
                                     "geolocation lookups")
    parser.add_argument("--executable",
                        help="An executable file performing lookups",
                        required=True)
    parser.add_argument("--database",
                        help="A path to database file",
                        required=True)
    args = parser.parse_args()

    # Start the process
    process = subprocess.Popen(
        [args.executable, args.database],
        bufsize=1,
        universal_newlines=True,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE)
    wait_ready(process)

    # Request to load the database
    load_time, memory_usage = send_load_command(process)
    print("Database loaded",
          "Memory usage:", format_memory_usage(memory_usage),
          "Load time:", format_time(load_time))

    # Define a few test cases
    test_suite = [
        TestData('1.0.0.0', 'US', 'Los Angeles'),
        TestData('71.6.28.0', 'US', 'San Jose'),
        TestData('71.6.28.255', 'US', 'San Jose'),
        TestData('71.6.29.0', 'US', 'Concord'),
        TestData('53.103.144.0', 'DE', 'Stuttgart'),
        TestData('53.255.255.255', 'DE', 'Stuttgart'),
        TestData('54.0.0.0', 'US', 'Rahway'),
        TestData('223.255.255.255', 'AU', 'Brisbane'),
        TestData('5.44.16.0', 'GB', 'Hastings'),
        TestData('8.24.99.0', 'US', 'Hastings'),
    ]

    # Initialize variables for resource tracking
    sum_lookup_time = 0
    max_memory_usage = 0
    success_count = 0

    # Perform test cases
    for test in test_suite:
        country_code, city, lookup_time, memory_usage = send_lookup_command(
                                                                    process,
                                                                    test.ip)

        correct = (country_code == test.country and city == test.city)

        if correct:
            print("OK   ",
                  test.ip,
                  country_code,
                  city,
                  "Memory usage:", format_memory_usage(memory_usage),
                  "Lookup time:", format_time(lookup_time)
                  )

            # Track the resource usage
            success_count += 1
            max_memory_usage = max(memory_usage, max_memory_usage)
            sum_lookup_time += lookup_time

        else:
            print("FAIL ",
                  test.ip,
                  country_code,
                  city,
                  "Expected:",
                  test.country,
                  test.city,
                  "Memory Usage:",
                  format_memory_usage(memory_usage),
                  "Lookup time:", format_time(lookup_time)
                  )

    # Calculate the points
    if success_count > 0:
        # Convert units
        avg_lookup_time_ms = ns_to_ms(sum_lookup_time / success_count)
        load_time_ms = ns_to_ms(load_time)
        max_memory_usage_mb = b_to_mb(max_memory_usage)

        # Calculate final points
        points = (load_time_ms +
                  max_memory_usage_mb * 10 +
                  avg_lookup_time_ms * 1000)

        print("Final points for", success_count, "measurements: ", points)
    else:
        print("Unable to calculate points without any successfull lookups")

    send_exit_command(process)
    process.wait()
