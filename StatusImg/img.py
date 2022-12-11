#!/usr/bin/env python3

import math
import colorsys
from typing import Optional, Tuple, Iterable

import cairo


ColorTriple = Tuple[float, float, float]


def color_triple(r: int, g: int, b: int) -> ColorTriple:
    return (r / 0xff, g / 0xff, b / 0xff)


COLOR_LOST = color_triple(0x6c, 0x75, 0x7d)
COLOR_SUCCESS = color_triple(0x28, 0xa7, 0x45)
COLOR_UNKNOWN = color_triple(0xff, 0xc1, 0x07)
COLOR_LINE_BACKGROUND = color_triple(0xf6, 0xf8, 0xfa)
COLOR_TEXT = color_triple(0x00, 0x00, 0x00)
YEAR_B = 2015
YEAR_E = 2022 + 1
DAY_E = 25 + 1
YEAR_WIDTH = 55
LEFT_MARGIN = 15
DAY_ROW_HEIGHT = 16
DAY_BOX_HEIGHT = 12
ProgressColor = Optional[ColorTriple]
IMG_WIDTH = LEFT_MARGIN + (YEAR_E - YEAR_B) * YEAR_WIDTH


def darken(rgb: ColorTriple) -> ColorTriple:
    (h, s, v) = colorsys.rgb_to_hsv(*rgb)
    return colorsys.hsv_to_rgb(h, s, v * 0.8)


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

    progress.mark_progress(COLOR_SUCCESS, 2018, range(1, 8))
    progress.mark_progress(COLOR_UNKNOWN, 2018, [6])

    progress.mark_progress(COLOR_SUCCESS, 2019, [1, 2, 3, 5])
    progress.mark_progress(COLOR_UNKNOWN, 2019, [9])
    progress.mark_progress(COLOR_UNKNOWN, 2019, [6], [1])

    progress.mark_progress(COLOR_SUCCESS, 2020, range(1, 10))
    progress.mark_progress(COLOR_SUCCESS, 2020, [10], [1])
    progress.mark_progress(COLOR_SUCCESS, 2020, [12])
    progress.mark_progress(COLOR_SUCCESS, 2020, [13, 14], [1])
    progress.mark_progress(COLOR_SUCCESS, 2020, [15])

    progress.mark_progress(COLOR_SUCCESS, 2021, range(1, 18))
    progress.mark_progress(COLOR_SUCCESS, 2021, [20])
    progress.mark_progress(COLOR_SUCCESS, 2021, [25], [1])

    progress.mark_progress(COLOR_SUCCESS, 2022, range(1, 7))
    progress.mark_progress(COLOR_SUCCESS, 2022, [8, 9, 10])

    with cairo.SVGSurface('StatusImg.svg', IMG_WIDTH, 435) as surface:
        cnt = cairo.Context(surface)

        cnt.set_source_rgb(*COLOR_TEXT)
        for (y_idx, y) in enumerate(range(YEAR_B, YEAR_E)):
            cnt.move_to(25 + y_idx * YEAR_WIDTH, 30)
            cnt.save()
            cnt.rotate(-math.pi / 2)
            cnt.show_text(str(y))
            cnt.restore()

        for (d_idx, d) in enumerate(range(1, DAY_E)):
            if d % 2 == 0:
                rectangle_params = (0, 33 + d_idx * DAY_ROW_HEIGHT, IMG_WIDTH, DAY_BOX_HEIGHT)
                cnt.set_source_rgb(*COLOR_LINE_BACKGROUND)
                cnt.rectangle(*rectangle_params)
                cnt.fill()
            cnt.set_source_rgb(*COLOR_TEXT)
            cnt.move_to(0, 42 + d_idx * DAY_ROW_HEIGHT)
            cnt.show_text(str(d).rjust(2))

        for (d_idx, d) in enumerate(range(1, DAY_E)):
            for (y_idx, y) in enumerate(range(YEAR_B, YEAR_E)):
                for (part, part_offset) in [(1, LEFT_MARGIN), (2, 37)]:
                    if color := progress.get(y, d, part):
                        rectangle_params = (part_offset + y_idx * YEAR_WIDTH, 33 + d_idx * DAY_ROW_HEIGHT, 20, DAY_BOX_HEIGHT)
                        cnt.set_source_rgb(*color)
                        cnt.rectangle(*rectangle_params)
                        cnt.fill()
                        cnt.set_source_rgb(*darken(color))
                        cnt.rectangle(*rectangle_params)
                        cnt.stroke()

    print('done')


if __name__ == "__main__":
    main()
