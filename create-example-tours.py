#!/usr/bin/env python3

import csv
import json
import os
from copy import deepcopy
from dataclasses import asdict, dataclass
from re import sub
from typing import List

BASE_URL = "https://growcult.uber.space/api/examples/"


@dataclass
class InfoNode:
    type: str
    name: str
    text: str
    audio: str


@dataclass
class Location:
    lat: float
    lon: float
    hints: List[InfoNode]


@dataclass
class Image:
    url: str
    description: str


@dataclass
class POI:
    id: str
    name: str
    location: Location
    alternate_locations: List[Location]
    images: List[Image]
    info_nodes: List[InfoNode]


@dataclass
class Tour:
    id: str
    name: str
    pois: List[POI]


def read_pois(csv_path):
    with open(csv_path, "r") as f:
        reader = csv.reader(f, delimiter=';', quotechar='"')
        next(reader)  # skip first row
        for row in reader:
            id_, lat, lon, url, name, lat_name, date, origin, leaves, blossoms, fruits = row
            yield POI(id_, name, get_location(lat, lon, lat_name), [], [
                get_image(name)
            ], [
                InfoNode("latin-name", "Lateinischer Name", lat_name, None),
                InfoNode("origin", "Lateinischer Name", origin, None),
                InfoNode("leaves", "Blätter", leaves, None),
                InfoNode("blossoms", "Blüten", blossoms, None),
                InfoNode("fruits", "Früchte", fruits, None),
                InfoNode("relation-to-leipzig", "Verbindung zu Leipzig", get_relation(origin), None),
            ])


def kebab(s):
  return '-'.join(
    sub(r"(\s|_|-|')+"," ",
    sub(r"[A-Z]{2,}(?=[A-Z][a-z]+[0-9]*|\b)|[A-Z]?[a-z]+[0-9]*|[A-Z]|[0-9]+",
    lambda mo: ' ' + mo.group(0).lower(), s)).split())


def get_location(lat, lon, lat_name):
    leave_image = kebab(lat_name) + "-leaf.jpg"
    hints = []
    if os.path.exists(os.path.join("examples", leave_image)):
        hints.append(InfoNode("leaves", "Blätter", f'<img src="{BASE_URL + leave_image}">', None))
    return Location(lat, lon, hints)


def get_pois_for_game(pois):
    pois = deepcopy(pois)
    result = pois[:3]
    pois = pois[3:]
    count_alt_locs = [4, 6, 3]
    for k, poi in enumerate(result):
        alt_locs = []
        for _ in range(count_alt_locs[k]):
            loc = pois.pop(0).location
            alt_loc = deepcopy(poi.location)
            alt_loc.lon = loc.lon
            alt_loc.lat = loc.lat
            alt_locs.append(alt_loc)
        poi.alternate_locations = alt_locs
    return result


def get_image(name):
    if "Silber" in name:
        return Image(BASE_URL + "tilia-tomentosa-petiolaris.jpg", name)
    elif "Pappel" in name:
        return Image(BASE_URL + "populus-nigra-italica.jpg", name)
    elif "Esche" in name:
        return Image(BASE_URL + "fraxinus-excelsior-pendula.jpg", name)
    else:
        return Image(BASE_URL + "populus-nigra-italica.jpg", name)


def get_relation(origin):
    with open("raw/Italien.html") as f:
        return f.read()


def save_tour_for_website(pois):
    tour = Tour("website", "Johannapark", pois)
    with open("examples/tour-website.json", "w") as f:
        json.dump(asdict(tour), f, indent=2)


def save_tour_for_game(pois):
    pois = get_pois_for_game(pois)
    tour = Tour("game", "Johannapark", pois)
    with open("examples/tour-game.json", "w") as f:
        json.dump(asdict(tour), f, indent=2)


pois = list(read_pois("raw/treelist.csv"))
save_tour_for_website(pois)
save_tour_for_game(pois)
