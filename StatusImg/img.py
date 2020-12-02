#!/usr/bin/env python3

import math

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


def main() -> None:
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
                    rectangle_params = (part_offset + y_idx * YEAR_WIDTH, 33 + d_idx * DAY_HEIGHT, 20, 12)
                    if y == 2015:
                        if d < 15:
                            if d < 8:
                                cnt.set_source_rgb(*COLOR_LOST)
                            else:
                                cnt.set_source_rgb(*COLOR_SUCCESS)
                            cnt.rectangle(*rectangle_params)
                            cnt.fill()

                    if y == 2016:
                        if (d < 22) or (d == 22 and part == 1):
                            cnt.set_source_rgb(*COLOR_LOST)
                            cnt.rectangle(*rectangle_params)
                            cnt.fill()

                    if y == 2017:
                        if d == 18 and part == 1:
                            cnt.set_source_rgb(*COLOR_SUCCESS)
                            cnt.rectangle(*rectangle_params)
                            cnt.fill()
                        elif d < 18:
                            if d == 17:
                                cnt.set_source_rgb(*COLOR_LOST)
                            elif (d == 8) or (d == 13 and part == 2) or (d == 16 and part == 2):
                                cnt.set_source_rgb(*COLOR_UNKNOWN)
                            else:
                                cnt.set_source_rgb(*COLOR_SUCCESS)
                            cnt.rectangle(*rectangle_params)
                            cnt.fill()

                    if y == 2018:
                        if d < 7:
                            if d == 6:
                                cnt.set_source_rgb(*COLOR_UNKNOWN)
                            else:
                                cnt.set_source_rgb(*COLOR_SUCCESS)
                            cnt.rectangle(*rectangle_params)
                            cnt.fill()

                    if y == 2019:
                        if d == 6 and part == 1:
                            cnt.set_source_rgb(*COLOR_TODO)
                            cnt.rectangle(*rectangle_params)
                            cnt.fill()
                        if d in [1, 2, 3, 5, 9]:
                            if (d == 9) or (d == 2 and part == 2):
                                cnt.set_source_rgb(*COLOR_TODO)
                            else:
                                cnt.set_source_rgb(*COLOR_SUCCESS)
                            cnt.rectangle(*rectangle_params)
                            cnt.fill()

                    if y == 2020:
                        if d <= 1 or d == 2 and part == 1:
                            cnt.set_source_rgb(*COLOR_SUCCESS)
                            cnt.rectangle(*rectangle_params)
                            cnt.fill()

    print('done')


if __name__ == "__main__":
    main()
