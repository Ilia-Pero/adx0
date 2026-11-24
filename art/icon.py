"""Draws the source of the icon: what an ad is multiplied by.

Run: python art/icon.py, then npx tauri icon art/icon.png -o src-tauri/icons.
The standard library is enough, so there is no Pillow for the sake of one picture.
"""

import struct
import zlib
from pathlib import Path

SIZE = 1024
HALF = SIZE // 2
STROKE = 46
CROSS = (324, HALF, 98)
ZERO = (688, HALF, 112, 200)
TOP = (20, 148, 136)
BOTTOM = (12, 92, 88)
WHITE = (255, 255, 255)


def box(x: float, y: float, hw: float, hh: float, radius: float) -> float:
    """The distance from the point to the rounded rectangle around zero: negative inside."""
    dx = abs(x) - (hw - radius)
    dy = abs(y) - (hh - radius)
    outside = (max(dx, 0.0) ** 2 + max(dy, 0.0) ** 2) ** 0.5
    return outside + min(max(dx, dy), 0.0) - radius


def cover(distance: float) -> float:
    """The share of the pixel covered by the figure, by the distance to its edge."""
    return min(max(0.5 - distance, 0.0), 1.0)


def diagonal(x: float, y: float, half: float) -> float:
    """The distance from the point to the segment from (-half, -half) to (half, half)."""
    along = min(max((x + y) / 2, -half), half)
    return ((x - along) ** 2 + (y - along) ** 2) ** 0.5


def cross(x: float, y: float) -> float:
    """The distance from the point to the sign of multiplication."""
    cx, cy, half = CROSS
    return min(diagonal(x - cx, y - cy, half), diagonal(x - cx, cy - y, half)) - STROKE


def zero(x: float, y: float) -> float:
    """The distance from the point to the zero."""
    cx, cy, hw, hh = ZERO
    return abs(box(x - cx, y - cy, hw, hh, hw)) - STROKE


def pixel(x: float, y: float) -> tuple[int, int, int, int]:
    """The colour of a point of the icon."""
    plate = cover(box(x - HALF, y - HALF, 448, 448, 200))
    mark = max(cover(cross(x, y)), cover(zero(x, y)))
    ground = (a + (b - a) * y / SIZE for a, b in zip(TOP, BOTTOM))
    red, green, blue = (round(g * (1 - mark) + w * mark) for g, w in zip(ground, WHITE))
    return red, green, blue, round(255 * plate)


def chunk(kind: bytes, body: bytes) -> bytes:
    return struct.pack(">I", len(body)) + kind + body + struct.pack(">I", zlib.crc32(kind + body))


def png() -> bytes:
    """PNG with the icon."""
    lines = []
    for y in range(SIZE):
        row = (pixel(x + 0.5, y + 0.5) for x in range(SIZE))
        lines.append(b"\x00" + bytes(value for dot in row for value in dot))
    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", struct.pack(">IIBBBBB", SIZE, SIZE, 8, 6, 0, 0, 0))
        + chunk(b"IDAT", zlib.compress(b"".join(lines), 9))
        + chunk(b"IEND", b"")
    )


if __name__ == "__main__":
    Path(__file__).with_name("icon.png").write_bytes(png())
