vessel_name: str = "KRI Bima Suci"
vessel_lat: float = -6.200000
vessel_lon: float = 106.816666
vessel_speed_knots: float = 100.5
vessel_speed_kmh: float = vessel_speed_knots * 1.852
print(f"Nama Kapal: {vessel_name}")
print(f"Koordinat Kapal: ({vessel_lat}, {vessel_lon})")
print(f"Kecepatan Kapal: {vessel_speed_knots} knots ({vessel_speed_kmh:.2f} km/h)")
