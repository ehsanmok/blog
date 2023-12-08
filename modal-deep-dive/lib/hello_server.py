from modal import Stub, web_endpoint

stub = Stub()


@stub.function()
@web_endpoint()
def f():
    return "Hello world!"
