import modal

stub = modal.Stub("another-app")
stub.square = modal.Function.from_name(
    "my-shared-app", "square"
)  # <-- NOTE: this must be deployed otherwise `modal run` won't find it


@stub.function()
def cube(x):
    return x * stub.square.remote(x)


@stub.local_entrypoint()
def main():
    assert cube.remote(42) == 74088
