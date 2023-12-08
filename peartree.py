import peartree as pt

feed = pt.get_representative_feed('/home/lolpro11/Documents/Catenary/algo/gtfs_rail.zip')

# Set a target time period to
# use to summarize impedance
start = 0 # 7:00 AM
end = 24*60*60  # 10:00 AM

# Converts feed subset into a directed
# network multigraph
G = pt.load_feed_as_graph(feed, start, end)
