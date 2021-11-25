import * as wasm from "hello-wasm-pack";

wasm.greet();

$(function() {
    var options = {
        target: '#svg_img',
        url: "/draw",
        type: "post",
        success: enableButton
    };
    $('#draw').ajaxForm(options);
});

function enableButton(responseText, statusText, xhr, $form)  {
    $('#fitplot').prop('disabled', false);
}