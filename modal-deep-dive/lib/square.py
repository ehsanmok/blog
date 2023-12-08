import modal

stub = modal.Stub("my-shared-app")


@stub.function()
def square(x):
    return x * x
