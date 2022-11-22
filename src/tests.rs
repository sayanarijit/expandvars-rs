use crate::env::FakeEnv;

use super::*;

#[test]
fn test_constant() {
    assert_eq!(expand("foo").unwrap(), "foo");
    assert_eq!(expand("$").unwrap(), "$");
    assert_eq!(expand("bar$").unwrap(), "bar$");
    assert_eq!(expand("{}{{}}").unwrap(), "{}{{}}");
}

#[test]
fn test_empty() {
    assert_eq!(expand("").unwrap(), "");
    assert_eq!(expand("$foo").unwrap(), "");
}

#[test]
fn test_simple() {
    let mut env = FakeEnv::empty().with("var", "value");

    assert_eq!(expand_with(&mut env, "$var").unwrap(), "value");
    assert_eq!(expand_with(&mut env, "${var}").unwrap(), "value");
}

#[test]
fn test_expandvars_combo() {
    let mut env = FakeEnv::empty().with("foo", "bar").with("biz", "buz");

    assert_eq!(expand_with(&mut env, "${foo}:$biz").unwrap(), "bar:buz");

    assert_eq!(expand_with(&mut env, "$foo$biz").unwrap(), "barbuz");

    assert_eq!(expand_with(&mut env, "${foo}$biz").unwrap(), "barbuz");

    assert_eq!(expand_with(&mut env, "$foo${biz}").unwrap(), "barbuz");

    assert_eq!(expand_with(&mut env, "${foo}-${biz}").unwrap(), "bar-buz");

    assert_eq!(expand_with(&mut env, "boo$biz").unwrap(), "boobuz");

    assert_eq!(expand_with(&mut env, "boo${biz}").unwrap(), "boobuz");
}

#[test]
fn test_expandvars_pid() {
    let pid = std::process::id().to_string();
    assert_eq!(expand("$$").unwrap(), pid);
    assert_eq!(expand("${$}").unwrap(), pid);
    assert_eq!(expand("PID( $$ )").unwrap(), format!("PID( {pid} )"));
}

#[test]
fn test_expandvars_get_default() {
    let mut env = FakeEnv::empty()
        .with("ALTERNATE", "Alternate")
        .with("EMPTY", "");

    assert_eq!(expand_with(&mut env, "${FOO-default}").unwrap(), "default");

    assert_eq!(
        expand_with(&mut env, "${EMPTY:-default}").unwrap(),
        "default"
    );

    assert_eq!(
        expand_with(&mut env, "${FOO:-foo}:${FOO-bar}").unwrap(),
        "foo:bar"
    );

    assert_eq!(
        expand_with(&mut env, "${FOO:-$ALTERNATE}").unwrap(),
        "Alternate"
    );

    assert_eq!(expand_with(&mut env, "${FOO:-}").unwrap(), "");

    assert_eq!(
        expand_with(&mut env, "${UNSET:-\\$foo}-\\$foo").unwrap(),
        "$foo-$foo"
    );
}

// // @patch.dict(env, {"ALTERNATE": "Alternate", "EMPTY": ""}, clear=True)
// // def test_expandvars_get_default():
// //     importlib.reload(expandvars)
// //
// //
// // @patch.dict(env, {"EMPTY": ""}, clear=True)
// // def test_expandvars_update_default():
// //     importlib.reload(expandvars)
// //
// assert_eq!(expand_with(&mut env, "${FOO:=}").unwrap(), ""
// assert_eq!(expand_with(&mut env, "${FOO=}").unwrap(), ""
// assert_eq!(expand_with(&mut env, "${EMPTY:=}").unwrap(), ""
// //
// //     del env["FOO"]
// //     del env["EMPTY"]
// //
// assert_eq!(expand_with(&mut env, "${FOO:=default}").unwrap(), "default"
// assert_eq!(expand_with(&mut env, "${FOO=default}").unwrap(), "default"
// assert_eq!(expand_with(&mut env, "${EMPTY:=default}").unwrap(), "default"
// //     assert env.get("FOO").unwrap(), "default"
// assert_eq!(expand_with(&mut env, "${FOO:=ignoreme}").unwrap(), "default"
// assert_eq!(expand_with(&mut env, "${EMPTY:=ignoreme}").unwrap(), "default"
// assert_eq!(expand_with(&mut env, "${FOO=ignoreme}:bar").unwrap(), "default:bar"
// //
// //
// // @patch.dict(env, {"FOO": "bar", "BUZ": "bar", "EMPTY": ""}, clear=True)
// // def test_expandvars_substitute():
// //     importlib.reload(expandvars)
// //
// assert_eq!(expand_with(&mut env, "${FOO:+foo}").unwrap(), "foo"
// assert_eq!(expand_with(&mut env, "${FOO+foo}").unwrap(), "foo"
// assert_eq!(expand_with(&mut env, "${BAR:+foo}").unwrap(), ""
// assert_eq!(expand_with(&mut env, "${BAR+foo}").unwrap(), ""
// assert_eq!(expand_with(&mut env, "${EMPTY:+foo}").unwrap(), ""
// assert_eq!(expand_with(&mut env, "${BAR:+}").unwrap(), ""
// assert_eq!(expand_with(&mut env, "${BAR+}").unwrap(), ""
// assert_eq!(expand_with(&mut env, "${BUZ:+foo}").unwrap(), "foo"
// assert_eq!(expand_with(&mut env, "${BUZ+foo}:bar").unwrap(), "foo:bar"
// assert_eq!(expand_with(&mut env, "${FOO:+${FOO};}").unwrap(), "bar;"
// assert_eq!(expand_with(&mut env, "${BAR:+${BAR};}").unwrap(), ""
// assert_eq!(expand_with(&mut env, "${BAR:+${EMPTY};}").unwrap(), ""
// assert_eq!(expand_with(&mut env, "${FOO:+\\$foo}-\\$foo").unwrap(), "$foo-$foo"
// //
// //
// // @patch.dict(env, {"FOO": "damnbigfoobar"}, clear=True)
// // def test_offset():
// //     importlib.reload(expandvars)
// //
// assert_eq!(expand_with(&mut env, "${FOO:3}").unwrap(), "nbigfoobar"
// assert_eq!(expand_with(&mut env, "${FOO: 4 }").unwrap(), "bigfoobar"
// assert_eq!(expand_with(&mut env, "${FOO:30}").unwrap(), ""
// assert_eq!(expand_with(&mut env, "${FOO:0}").unwrap(), "damnbigfoobar"
// assert_eq!(expand_with(&mut env, "${FOO:-3}:bar").unwrap(), "damnbigfoobar:bar"
// //
// //
// // @patch.dict(env, {"FOO": "damnbigfoobar"}, clear=True)
// // def test_offset_length():
// //     importlib.reload(expandvars)
// //
// assert_eq!(expand_with(&mut env, "${FOO:4:3}").unwrap(), "big"
// assert_eq!(expand_with(&mut env, "${FOO: 7:6 }").unwrap(), "foobar"
// assert_eq!(expand_with(&mut env, "${FOO:7: 100 }").unwrap(), "foobar"
// assert_eq!(expand_with(&mut env, "${FOO:0:100}").unwrap(), "damnbigfoobar"
// assert_eq!(expand_with(&mut env, "${FOO:70:10}").unwrap(), ""
// assert_eq!(expand_with(&mut env, "${FOO:1:0}").unwrap(), ""
// assert_eq!(expand_with(&mut env, "${FOO:0:}").unwrap(), ""
// assert_eq!(expand_with(&mut env, "${FOO::}").unwrap(), ""
// assert_eq!(expand_with(&mut env, "${FOO::5}").unwrap(), "damnb"
// assert_eq!(expand_with(&mut env, "${FOO:-3:1}:bar").unwrap(), "damnbigfoobar:bar"
// //
// //
// // @patch.dict(env, {"FOO": "X", "X": "foo"}, clear=True)
// // def test_expandvars_indirection():
// //     importlib.reload(expandvars)
// //
// assert_eq!(expand_with(&mut env, "${!FOO}:${FOO}").unwrap(), "foo:X"
// assert_eq!(expand_with(&mut env, "${!FOO-default}").unwrap(), "foo"
// assert_eq!(expand_with(&mut env, "${!BAR-default}").unwrap(), "default"
// assert_eq!(expand_with(&mut env, "${!X-default}").unwrap(), "default"
// //
// //
// // @patch.dict(env, {"FOO": "foo", "BAR": "bar"}, clear=True)
// // def test_escape():
// //     importlib.reload(expandvars)
// //
// assert_eq!(expand_with(&mut env, "\\$FOO\\$BAR").unwrap(), "$FOO$BAR"
// assert_eq!(expand_with(&mut env, "\\\\$FOO").unwrap(), "\\foo"
// assert_eq!(expand_with(&mut env, "$FOO\\$BAR").unwrap(), "foo$BAR"
// assert_eq!(expand_with(&mut env, "\\$FOO$BAR").unwrap(), "$FOObar"
// assert_eq!(expand_with(&mut env, "$FOO" "\\" "\\" "\\" "$BAR") == ("foo" "\\" "$BAR")
// assert_eq!(expand_with(&mut env, "$FOO\\$").unwrap(), "foo$"
// assert_eq!(expand_with(&mut env, "$\\FOO").unwrap(), "$\\FOO"
// assert_eq!(expand_with(&mut env, "$\\$FOO").unwrap(), "$$FOO"
// assert_eq!(expand_with(&mut env, "\\$FOO").unwrap(), "$FOO"
// //     assert (
// //         expandvars.expandvars("D:\\\\some\\windows\\path")
// //         == "D:\\\\some\\windows\\path"
// //     )
// //
// //
// // @patch.dict(env, {}, clear=True)
// // def test_corner_cases():
// //     importlib.reload(expandvars)
// //
// assert_eq!(expand_with(&mut env, "${FOO:-{}}{}{}{}{{}}").unwrap(), "{}{}{}{}{{}}"
// assert_eq!(expand_with(&mut env, "${FOO-{}}{}{}{}{{}}").unwrap(), "{}{}{}{}{{}}"
// //
// //
// // @patch.dict(env, {}, clear=True)
// // def test_strict_parsing():
// //     importlib.reload(expandvars)
// //
// //     with pytest.raises(
// //         expandvars.ExpandvarsException, match="FOO: parameter null or not set"
// //     ) as e:
// //         expandvars.expandvars("${FOO:?}")
// //     assert isinstance(e.value, expandvars.ParameterNullOrNotSet)
// //
// //     with pytest.raises(
// //         expandvars.ExpandvarsException, match="FOO: parameter null or not set"
// //     ) as e:
// //         expandvars.expandvars("${FOO?}")
// //     assert isinstance(e.value, expandvars.ParameterNullOrNotSet)
// //
// //     with pytest.raises(expandvars.ExpandvarsException, match="FOO: custom error") as e:
// //         expandvars.expandvars("${FOO:?custom error}")
// //     assert isinstance(e.value, expandvars.ParameterNullOrNotSet)
// //
// //     with pytest.raises(expandvars.ExpandvarsException, match="FOO: custom error") as e:
// //         expandvars.expandvars("${FOO?custom error}")
// //     assert isinstance(e.value, expandvars.ParameterNullOrNotSet)
// //
// //     env.update({"FOO": "foo"})
// //
// assert_eq!(expand_with(&mut env, "${FOO:?custom err}").unwrap(), "foo"
// assert_eq!(expand_with(&mut env, "${FOO?custom err}:bar").unwrap(), "foo:bar"
// //
// //
// // @patch.dict(env, {"FOO": "foo"}, clear=True)
// // def test_missing_escapped_character():
// //     importlib.reload(expandvars)
// //
// //     with pytest.raises(expandvars.ExpandvarsException) as e:
// //         expandvars.expandvars("$FOO\\")
// //
// //     assert str(e.value).unwrap(), "$FOO\\: missing escaped character"
// //     assert isinstance(e.value, expandvars.MissingExcapedChar)
// //
// //
// // @patch.dict(env, {"FOO": "damnbigfoobar"}, clear=True)
// // def test_invalid_length_err():
// //     importlib.reload(expandvars)
// //
// //     with pytest.raises(
// //         expandvars.ExpandvarsException, match="FOO: -3: substring expression < 0"
// //     ) as e:
// //         expandvars.expandvars("${FOO:1:-3}")
// //     assert isinstance(e.value, expandvars.NegativeSubStringExpression)
// //
// //
// // @patch.dict(env, {"FOO": "damnbigfoobar"}, clear=True)
// // def test_bad_substitution_err():
// //     importlib.reload(expandvars)
// //
// //     with pytest.raises(expandvars.ExpandvarsException) as e:
// //         expandvars.expandvars("${FOO:}").unwrap(), ""
// //     assert str(e.value).unwrap(), "${FOO:}: bad substitution"
// //     assert isinstance(e.value, expandvars.BadSubstitution)
// //
// //     with pytest.raises(expandvars.ExpandvarsException) as e:
// //         expandvars.expandvars("${}").unwrap(), ""
// //     assert str(e.value).unwrap(), "${}: bad substitution"
// //     assert isinstance(e.value, expandvars.BadSubstitution)
// //
// //
// // @patch.dict(env, {"FOO": "damnbigfoobar"}, clear=True)
// // def test_brace_never_closed_err():
// //     importlib.reload(expandvars)
// //
// //     with pytest.raises(expandvars.ExpandvarsException) as e:
// //         expandvars.expandvars("${FOO:")
// //     assert str(e.value).unwrap(), "${FOO:: missing '}'"
// //     assert isinstance(e.value, expandvars.MissingClosingBrace)
// //
// //     with pytest.raises(expandvars.ExpandvarsException) as e:
// //         expandvars.expandvars("${FOO}${BAR")
// //     assert str(e.value).unwrap(), "${FOO}${BAR: missing '}'"
// //     assert isinstance(e.value, expandvars.MissingClosingBrace)
// //
// //     with pytest.raises(expandvars.ExpandvarsException) as e:
// //         expandvars.expandvars("${FOO?")
// //     assert str(e.value).unwrap(), "${FOO?: missing '}'"
// //     assert isinstance(e.value, expandvars.ExpandvarsException)
// //
// //     with pytest.raises(expandvars.ExpandvarsException) as e:
// //         expandvars.expandvars("${FOO:1")
// //     assert str(e.value).unwrap(), "${FOO:1: missing '}'"
// //     assert isinstance(e.value, expandvars.MissingClosingBrace)
// //
// //     with pytest.raises(expandvars.ExpandvarsException) as e:
// //         expandvars.expandvars("${FOO:1:2")
// //     assert str(e.value).unwrap(), "${FOO:1:2: missing '}'"
// //     assert isinstance(e.value, expandvars.MissingClosingBrace)
// //
// //     with pytest.raises(expandvars.ExpandvarsException) as e:
// //         expandvars.expandvars("${FOO+")
// //     assert str(e.value).unwrap(), "${FOO+: missing '}'"
// //     assert isinstance(e.value, expandvars.MissingClosingBrace)
// //
// //     with pytest.raises(expandvars.ExpandvarsException) as e:
// //         expandvars.expandvars("${FOO-")
// //     assert str(e.value).unwrap(), "${FOO-: missing '}'"
// //     assert isinstance(e.value, expandvars.MissingClosingBrace)
// //
// //     with pytest.raises(expandvars.ExpandvarsException) as e:
// //         expandvars.expandvars("${FOO-{{}")
// //     assert str(e.value).unwrap(), "${FOO-{{}: missing '}'"
// //     assert isinstance(e.value, expandvars.MissingClosingBrace)
// //
// //
// //
// // @patch.dict(env, {"FOO": "damnbigfoobar"}, clear=True)
// // def test_invalid_operand_err():
// //     importlib.reload(expandvars)
// //
// //     oprnds = "@#$%^&*()_'\""
// //
// //     for o in oprnds:
// //         with pytest.raises(expandvars.ExpandvarsException) as e:
// //             expandvars.expandvars("${{FOO:0:{0}}}".format(o))
// //         assert str(e.value) == ("FOO: operand expected (error token is {0})").format(
// //             repr(o)
// //         )
// //         assert isinstance(e.value, expandvars.OperandExpected)
// //
// //         with pytest.raises(expandvars.ExpandvarsException) as e:
// //             expandvars.expandvars("${{FOO:{0}:{0}}}".format(o))
// //         assert str(e.value) == ("FOO: operand expected (error token is {0})").format(
// //             repr(o)
// //         )
// //         assert isinstance(e.value, expandvars.OperandExpected)
// //
// //
// // @pytest.mark.parametrize("var_symbol", ["%", "&", "£", "="])
// // def test_expand_var_symbol(var_symbol):
// //     importlib.reload(expandvars)
// //
// //     assert (
// //         expandvars.expand(
// //             var_symbol + "{FOO}", environ={"FOO": "test"}, var_symbol=var_symbol
// //         )
// //         == "test"
// //     )
// //     assert (
// //         expandvars.expand(var_symbol + "FOO", environ={}, var_symbol=var_symbol).unwrap(), ""
// //     )
// //     assert (
// //         expandvars.expand(
// //             var_symbol + "{FOO:-default_value}", environ={}, var_symbol=var_symbol
// //         )
// //         == "default_value"
// //     )
// //     with pytest.raises(expandvars.ParameterNullOrNotSet):
// //         expandvars.expand(var_symbol + "{FOO:?}", environ={}, var_symbol=var_symbol)
// //
// //     assert (
// //         expandvars.expand(
// //             var_symbol + "{FOO},$HOME", environ={"FOO": "test"}, var_symbol=var_symbol
// //         )
// //         == "test,$HOME"
// //     )
// //
// // @patch.dict(env, {"FOO": "bar"}, clear=True)
// // def test_expandvars_from_file():
// //     importlib.reload(expandvars)
// //
// //     with open("tests/data/foo.txt") as f:
// //         assert expandvars.expandvars(f).unwrap(), "bar:bar"
// //
