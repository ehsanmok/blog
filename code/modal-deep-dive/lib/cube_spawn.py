import time
import modal
from modal.functions import FunctionCall

stub = modal.Stub("cube-spawn")
stub.square = modal.Function.from_name(
    "my-shared-app", "square"
)  # <-- NOTE: this must be deployed otherwise `modal run` won't find it


@stub.function()
def spawn_square(x):
    call = stub.square.spawn(x)
    return {"call_id": call.object_id}


@stub.function()
def poll(call_id):
    fcall = FunctionCall.from_id(call_id)
    try:
        # 5 seconds timeout to simulate a long running job
        ret = fcall.get(timeout=5)
    except TimeoutError:
        print("waiting for result")
        return

    return ret


@stub.local_entrypoint()
def cube():
    call = spawn_square.remote(42)
    call_id = call["call_id"]
    assert call_id is not None
    ret = poll.remote(call_id)
    assert ret * 42 == 74088
