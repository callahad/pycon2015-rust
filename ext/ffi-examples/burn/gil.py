#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from threading import Thread


def run(i):
    print("Thread {} starting".format(i))

    x = 0
    while x < 10000000:
        x += 1

    print("Thread {} returned".format(i))


def triple(x):
    thread_list = []

    for i in range(10):
        t = Thread(target=run, args=(i,))
        thread_list.append(t)

    for thread in thread_list:
        thread.start()

    for thread in thread_list:
        thread.join()

    return x * 3


if __name__ == '__main__':
    print(triple(3))
