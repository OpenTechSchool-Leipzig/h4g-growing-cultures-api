#!/usr/bin/env python3

import csv
import json
from dataclasses import asdict, dataclass
from typing import List


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
            yield POI(id_, name, Location(lat, lon, []), [
                Image("https://growcult.uber.space/api/examples/populus-nigra-italica.jpg", name),
            ], [
                InfoNode("latin-name", "Lateinischer Name", lat_name, None),
                InfoNode("origin", "Lateinischer Name", origin, None),
                InfoNode("leaves", "Blätter", leaves, None),
                InfoNode("blossoms", "Blüten", blossoms, None),
                InfoNode("fruits", "Früchte", fruits, None),
                InfoNode("relation-to-leipzig", "Verbindung zu Leipzig", get_relation(origin), None),
            ])


def get_relation(origin):
    with open("raw/Italien.html") as f:
        return f.read()


def save_tour_for_website(pois):
    tour = Tour("website", "Johannapark", pois)
    with open("examples/tour-website.json", "w") as f:
        json.dump(asdict(tour), f, indent=2)


pois = list(read_pois("raw/treelist.csv"))
save_tour_for_website(pois)
