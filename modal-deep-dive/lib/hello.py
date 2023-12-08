import modal

stub = modal.Stub("hello")


@stub.function()
def hello():
    print("hello, world!")
