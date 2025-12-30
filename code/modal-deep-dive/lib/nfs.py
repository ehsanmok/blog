import modal

stub = modal.Stub()
volume = modal.NetworkFileSystem.new()


@stub.function(network_file_systems={"/root/foo": volume})
def f():
    pass


@stub.function(network_file_systems={"/root/goo": volume})
def g():
    pass
