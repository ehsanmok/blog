import pathlib
import modal

p = pathlib.Path("/root/foo/bar.txt")

stub = modal.Stub()
stub.volume = modal.Volume.new()


@stub.function(volumes={"/root/foo": stub.volume})
def f():
    p.write_text("hello")
    print(f"Created {p=}")
    stub.volume.commit()  # Persist changes
    print(f"Committed {p=}")
