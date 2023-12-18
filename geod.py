def Inverse(self, lat1, lon1, lat2, lon2, outmask = GeodesicCapability.STANDARD):
    """Solve the inverse geodesic problem

    :param lat1: latitude of the first point in degrees
    :param lon1: longitude of the first point in degrees
    :param lat2: latitude of the second point in degrees
    :param lon2: longitude of the second point in degrees
    :param outmask: the :ref:`output mask <outmask>`
    :return: a :ref:`dict`

    Compute geodesic between (*lat1*, *lon1*) and (*lat2*, *lon2*).
    The default value of *outmask* is STANDARD, i.e., the *lat1*,
    *lon1*, *azi1*, *lat2*, *lon2*, *azi2*, *s12*, *a12* entries are
    returned.

    """

    a12, s12, salp1,calp1, salp2,calp2, m12, M12, M21, S12 = self._GenInverse(
      lat1, lon1, lat2, lon2, outmask)
    outmask &= Geodesic.OUT_MASK
    if outmask & Geodesic.LONG_UNROLL:
      lon12, e = Math.AngDiff(lon1, lon2)
      lon2 = (lon1 + lon12) + e
    else:
      lon2 = Math.AngNormalize(lon2)
    result = {'lat1': Math.LatFix(lat1),
              'lon1': lon1 if outmask & Geodesic.LONG_UNROLL else
              Math.AngNormalize(lon1),
              'lat2': Math.LatFix(lat2),
              'lon2': lon2}
    result['a12'] = a12
    if outmask & Geodesic.DISTANCE: result['s12'] = s12
    if outmask & Geodesic.AZIMUTH:
      result['azi1'] = Math.atan2d(salp1, calp1)
      result['azi2'] = Math.atan2d(salp2, calp2)
    if outmask & Geodesic.REDUCEDLENGTH: result['m12'] = m12
    if outmask & Geodesic.GEODESICSCALE:
      result['M12'] = M12; result['M21'] = M21
    if outmask & Geodesic.AREA: result['S12'] = S12
    return result
def Direct(self, lat1, lon1, azi1, s12,
             outmask = GeodesicCapability.STANDARD):
    """Solve the direct geodesic problem

    :param lat1: latitude of the first point in degrees
    :param lon1: longitude of the first point in degrees
    :param azi1: azimuth at the first point in degrees
    :param s12: the distance from the first point to the second in
      meters
    :param outmask: the :ref:`output mask <outmask>`
    :return: a :ref:`dict`

    Compute geodesic starting at (*lat1*, *lon1*) with azimuth *azi1*
    and length *s12*.  The default value of *outmask* is STANDARD, i.e.,
    the *lat1*, *lon1*, *azi1*, *lat2*, *lon2*, *azi2*, *s12*, *a12*
    entries are returned.

    """

    a12, lat2, lon2, azi2, s12, m12, M12, M21, S12 = self._GenDirect(
      lat1, lon1, azi1, False, s12, outmask)
    outmask &= Geodesic.OUT_MASK
    result = {'lat1': Math.LatFix(lat1),
              'lon1': lon1 if outmask & Geodesic.LONG_UNROLL else
              Math.AngNormalize(lon1),
              'azi1': Math.AngNormalize(azi1),
              's12': s12}
    result['a12'] = a12
    if outmask & Geodesic.LATITUDE: result['lat2'] = lat2
    if outmask & Geodesic.LONGITUDE: result['lon2'] = lon2
    if outmask & Geodesic.AZIMUTH: result['azi2'] = azi2
    if outmask & Geodesic.REDUCEDLENGTH: result['m12'] = m12
    if outmask & Geodesic.GEODESICSCALE:
      result['M12'] = M12; result['M21'] = M21
    if outmask & Geodesic.AREA: result['S12'] = S12
    return result