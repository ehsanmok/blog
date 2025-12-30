import modal

stub = modal.Stub("hello")


@stub.function()
def hello():
    print("hello, world!")


@stub.function()
def hello2():
    print("hello, world second time!")


@stub.local_entrypoint()
def main():
    # hello.local()  # semantically is `hello()`
    print(hello.remote())
