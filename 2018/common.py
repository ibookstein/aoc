import requests
import requests_cache


_MY_SESSION = '53616c7465645f5f1d6ee3b5731560dbc4effaf373eb87d2acbac472596864ab86bc7b12eb982631cb18d9dd919e7426'
requests_cache.install_cache('aoc2018_inputs')


def get_input(day):
    url = 'https://adventofcode.com/2018/day/{}/input'.format(day)
    return requests.get(url, cookies={'session': _MY_SESSION}).text


def get_input_lines(day):
    return get_input(day).split('\n')[:-1]
