from renfe.timetable import get_timetable

def test_renfe_cli_should_get_non_empty_timetable():
    tt = get_timetable("MADRI", "BARCE", 1, "firefox", 10)

    assert type(tt) == list
    assert any(tt)
