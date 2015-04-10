#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from threading import Thread


def count(i):
    print("Thread {} starting".format(i))

    x = 0
    while x < 2**23:
        x += 1

    print("Thread {} returned".format(i))


def main():
    thread_list = []

    for i in range(10):
        t = Thread(target=count, args=(i,))
        thread_list.append(t)

    for thread in thread_list:
        thread.start()

    for thread in thread_list:
        thread.join()

    print("Completely done")


if __name__ == '__main__':
    main()
