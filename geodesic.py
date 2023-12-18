import math
from math import sin, asin, radians, tan, atan
from geographiclib.geodesic import Geodesic

'''
The results returned by Geodesic.Direct, Geodesic.Inverse, GeodesicLine.Position, etc., return a dictionary with some of the following 12 fields set:

lat1 = φ1, latitude of point 1 (degrees)
lon1 = λ1, longitude of point 1 (degrees)
azi1 = α1, azimuth of line at point 1 (degrees)
lat2 = φ2, latitude of point 2 (degrees)
lon2 = λ2, longitude of point 2 (degrees)
azi2 = α2, (forward) azimuth of line at point 2 (degrees)
s12 = s12, distance from 1 to 2 (meters)
a12 = σ12, arc length on auxiliary sphere from 1 to 2 (degrees)
m12 = m12, reduced length of geodesic (meters)
M12 = M12, geodesic scale at 2 relative to 1 (dimensionless)
M21 = M21, geodesic scale at 1 relative to 2 (dimensionless)
S12 = S12, area between geodesic and equator (meters2)
'''

geod = Geodesic.WGS84
R = 6378137.0

debug = True
def dd_to_dms(degs):
    neg = degs < 0
    degs = (-1) ** neg * degs
    degs, d_int = math.modf(degs)
    mins, m_int = math.modf(60 * degs)
    secs        =           60 * mins
    return neg, d_int, m_int, secs

def dmsStr(val):
    dms = dd_to_dms(val)
    return f"{'-' if dms[0] else ''}{dms[1]}° {dms[2]}\' {round(dms[3], 4)}\""

def PointToGeodesic(pA, pB, pP):
    while True:
        ap = geod.Inverse(pA[0], pA[1], pP[0], pP[1])
        ab = geod.Inverse(pA[0], pA[1], pB[0], pB[1])
        s_ap = ap['s12']
        A = ap['azi1'] - ab['azi1']

        s_px = R * asin( sin(s_ap / R) * sin(radians(A)) )
        s_ax = 2 * R * atan( sin(radians((90.0 + A) / 2.0)) / sin(radians((90.0 - A) / 2.0)) * tan((s_ap - s_px)/(2*R)) )

        pA2 = geod.Direct(pA[0], pA[1], ab['azi1'], s_ax)

        if debug:
            print(f"{dmsStr(pA2['lat2']):>22}, {dmsStr(pA2['lon2']):>22}, {round(s_ax, 4):>12}")

        if abs(s_ax) < 1e-2:
            break

        pA = (pA2['lat2'], pA2['lon2'])

    return pA

def Test(pA, pB, pP):
    print(pA, pB, pP)
    result = PointToGeodesic(pA, pB, pP)
    print( f"({dmsStr(result[0])}, {dmsStr(result[1])})" )
    print()

Test((52, 5), (51.4, 6), (52, 5.5))
Test((42, 29), (39, -77), (64, -22))
Test((42, 29), (-35, -70), (64, -22))
