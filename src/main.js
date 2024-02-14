const { invoke } = window.__TAURI__.tauri;

var table;

function tick() {
    
}

function rowToggleDisabled(row) {
    let email = row.querySelector("#chk-email");
    if (row.classList.contains("disabled-row")) {
        row.classList.remove("disabled-row");
        email.disabled = null;
    } else {
        row.classList.add("disabled-row");
        email.disabled = true;
    }
}

/********************** Listeners **********************/

function toggleClient(id, index) {
    let row = document.querySelector("#" + id);
    let value = row.querySelector("#chk-enabled")
    
    rowToggleDisabled(row);

    console.log(row, value.checked, row.getAttributeNames());
}

function toggleEmail(id, index) {
    let value = document.querySelectorAll(`#${id}>#chk-email`).checked;
    console.log(value);
}

function urlKeyDown(id, index) {
    console.log()
    let row = document.getElementById(id);
    let input = row.querySelector("#url");
    let enabled = row.querySelector("#" + id + " #chk-enabled");
    let newValue = input.value;

    if (newValue.length === 0) {
        enabled.checked = false;
        rowToggleDisabled(row);
    }

    else if (!enabled.checked && (newValue && newValue.length > 0)) {
        enabled.checked = true;
        row.classList.remove("disabled-row");
        console.log(row);
    }
}

function timeoutChanged(id, index) {
    let row = document.getElementById(id);
    let timeout = row.querySelector("#" + id + " #timeout");
    let value = timeout.value;

    
}

window.addEventListener("DOMContentLoaded", async () => {
    table = document.querySelector("#clients");

    let default_row = table.rows[1].cloneNode(true);
    default_row.classList.add("disabled-row");
    default_row.querySelector("#url").value = "";
    default_row.querySelector("#chk-enabled").checked = false;
    default_row.querySelector("#status").innerHTML = '<span id="status" style="color: gray">x</span>';
    default_row.classList.add("disabled-row");

    const newRow = (index) => {
        let row = default_row.cloneNode(true);
        let email = row.querySelector("#chk-email");
        email.onclick = () => toggleEmail(row.id, row.index);
        email.disabled = true;

        row.id = "client-" + index;
        row.index = index;
        row.querySelector("#chk-enabled").onclick = () => toggleClient(row.id, row.index);
        row.querySelector("#url").oninput = () => urlKeyDown(row.id, row.index);
        row.querySelector("#timeout").oninput = () => timeoutChanged(row.id, row.index);
        return row;
    };

    let first = document.querySelector("#client-0");
    first.querySelector("#chk-enabled").onclick = () => toggleClient("client-0", 0);
    first.querySelector("#chk-email").onclick = () => toggleEmail("client-0", 0);
    first.querySelector("#url").oninput = () => urlKeyDown(row.id, row.index);
    first.querySelector("#timeout").oninput = () => timeoutChanged(row.id, row.index);

    for (var i = 2; i < 120; i++) {
        let row = newRow(i);
        table.appendChild(row);
    }

    let smtpLogin = await invoke("smtp_login", {});
    console.log(smtpLogin);
});
