import base_encoding


def test_base64_encode():
    assert base_encoding.base64_encode("") == ""
    assert base_encoding.base64_encode("Many hands make light work.") == "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu"


def test_base64_decode():
    assert base_encoding.base64_decode("TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu") == "Many hands make light work."

    try:
        base_encoding.base64_decode("3q2+7w")
        assert False, "Should have raised an error for invalid padding"
    except Exception:
        assert True


if __name__ == "__main__":
    test_base64_encode()
    test_base64_decode()
    print("All tests passed!")
