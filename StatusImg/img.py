#!/usr/bin/env python3

import math
from typing import Optional, Tuple, Iterable

import cairo

COLOR_TODO = (0x00 / 0xff, 0x7b / 0xff, 0xff / 0xff)
COLOR_LOST = (0x6c / 0xff, 0x75 / 0xff, 0x7d / 0xff)
COLOR_SUCCESS = (0x28 / 0xff, 0xa7 / 0xff, 0x45 / 0xff)
COLOR_UNKNOWN = (0xff / 0xff, 0xc1 / 0xff, 0x07 / 0xff)
YEAR_B = 2015
YEAR_E = 2020 + 1
DAY_E = 25 + 1
YEAR_WIDTH = 55
DAY_HEIGHT = 14
ColorTriple = Tuple[float, float, float]
ProgressColor = Optional[ColorTriple]


class Progress:
    def __init__(self) -> None:
        self._progress = []  # dimensions: year, day, part
        for _ in enumerate(range(YEAR_B, YEAR_E)):
            day_sublist = []
            for __ in enumerate(range(1, DAY_E)):
                day_sublist.append([None, None])
            self._progress.append(day_sublist)

    def mark_progress(self, status: ProgressColor, year: int, days: Iterable[int], parts: Optional[Iterable[int]] = None) -> None:
        if parts is None:
            parts = [1, 2]

        for d in days:
            for p in parts:
                self._progress[year - YEAR_B][d - 1][p - 1] = status

    def get(self, year: int, day: int, part: int) -> ProgressColor:
        return self._progress[year - YEAR_B][day - 1][part - 1]


def main() -> None:
    progress = Progress()

    progress.mark_progress(COLOR_LOST, 2015, range(1, 8))
    progress.mark_progress(COLOR_SUCCESS, 2015, range(8, 15))

    progress.mark_progress(COLOR_LOST, 2016, range(1, 22))
    progress.mark_progress(COLOR_LOST, 2016, [22], [1])

    progress.mark_progress(COLOR_SUCCESS, 2017, range(1, 17))
    progress.mark_progress(COLOR_UNKNOWN, 2017, [8])
    progress.mark_progress(COLOR_UNKNOWN, 2017, [13], [2])
    progress.mark_progress(COLOR_UNKNOWN, 2017, [16], [2])
    progress.mark_progress(COLOR_LOST, 2017, [17])
    progress.mark_progress(COLOR_SUCCESS, 2017, [18], [1])

    progress.mark_progress(COLOR_SUCCESS, 2018, range(1, 6))
    progress.mark_progress(COLOR_UNKNOWN, 2018, [6])

    progress.mark_progress(COLOR_SUCCESS, 2019, [1, 2, 3, 5])
    progress.mark_progress(COLOR_TODO, 2019, [9])
    progress.mark_progress(COLOR_TODO, 2019, [2], [2])
    progress.mark_progress(COLOR_TODO, 2019, [6], [1])

    progress.mark_progress(COLOR_SUCCESS, 2020, range(1, 5))

    with cairo.SVGSurface('StatusImg.svg', 400, 400) as surface:
        cnt = cairo.Context(surface)

        for (y_idx, y) in enumerate(range(YEAR_B, YEAR_E)):
            cnt.move_to(25 + y_idx * YEAR_WIDTH, 30)
            cnt.save()
            cnt.rotate(-math.pi / 2)
            cnt.show_text(str(y))
            cnt.restore()

        for (d_idx, d) in enumerate(range(1, DAY_E)):
            cnt.move_to(0, 42 + d_idx * DAY_HEIGHT)
            cnt.show_text(str(d).rjust(2))

        for (d_idx, d) in enumerate(range(1, DAY_E)):
            for (y_idx, y) in enumerate(range(YEAR_B, YEAR_E)):
                for (part, part_offset) in [(1, 15), (2, 37)]:
                    if color := progress.get(y, d, part):
                        rectangle_params = (part_offset + y_idx * YEAR_WIDTH, 33 + d_idx * DAY_HEIGHT, 20, 12)
                        cnt.set_source_rgb(*color)
                        cnt.rectangle(*rectangle_params)
                        cnt.fill()

    print('done')


if __name__ == "__main__":
    main()
