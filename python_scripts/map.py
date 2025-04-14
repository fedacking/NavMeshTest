import json

with open("python_scripts/input/map.data", "r+") as file:
    data = json.load(file)

with open("python_scripts/output/merliot.fnav", "w+") as file:
    for thing in data["obstacles"]:
        vertices = thing["vertices"]
        if len(vertices) > 0:
            for vertex in vertices:
                file.write(f"{vertex.get('x', 0.0)};{vertex.get('y', 0.0)}|")
            file.write("\n")
    