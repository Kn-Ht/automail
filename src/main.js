const { invoke } = window.__TAURI__.tauri;

const dataStore = new Store(".settings.dat");

var table;
var smtpLogin;

const saveData = async () => {
    await invoke("save_state", {
        
    })
};

function tick() {
    console.log("tick");
}

/// Save user data to file
function saveData() {

}

function rowToggleEnabled(row, enable) {
    let email = row.querySelector("#chk-email");
    let timeout = row.querySelector("#timeout");

    if (enable) {
        row.classList.remove("disabled-row");
        timeout.disabled = null;
        email.disabled = null;
    } else {
        row.classList.add("disabled-row");
        timeout.disabled = true;
        email.disabled = true;
    }
}

/********************** Listeners **********************/

function toggleClient(id, index) {
    let row = document.querySelector("#" + id);
    rowToggleEnabled(row, row.querySelector("#chk-enabled").checked);
}

async function toggleEmail(id, index) {
    let row = document.querySelector("#" + id);
    let value = row.querySelector("#chk-email").value;
    console.log(value);
    if (value) {
        if (!smtpLogin) {
            // TODO: create window and ask for login details
            await invoke("open_settings_window", {});
        }
    }
}

function urlKeyDown(id, index) {
    console.log()
    let row = document.getElementById(id);
    let input = row.querySelector("#url");
    let enabled = row.querySelector("#" + id + " #chk-enabled");
    let newValue = input.value;

    if (newValue.length === 0) {
        enabled.checked = false;
        rowToggleEnabled(row, false);
    }

    else if (!enabled.checked && (newValue && newValue.length > 0)) {
        enabled.checked = true;
        rowToggleEnabled(row, true);
        console.log(row);
    }
}

function timeoutChanged(id, index) {
    let row = document.getElementById(id);
    let timeout = row.querySelector("#" + id + " #timeout");
    let value = timeout.value;

    // TODO
}

window.addEventListener("DOMContentLoaded", async () => {
    table = document.querySelector("#clients");

    let default_row = table.rows[1].cloneNode(true);
    default_row.classList.add("disabled-row");
    default_row.querySelector("#url").value = "";
    default_row.querySelector("#chk-enabled").checked = false;
    default_row.querySelector("#status").innerHTML = '<span id="status" style="color: gray">x</span>';
    default_row.querySelector("#timeout").disabled = true;
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

    smtpLogin = await invoke("smtp_login", {});

    const userData = await dataStore.get('userdata');

    if (userData) {
        smtpLogin = await invoke("make_user", {
            username: userData.username,
            password: userData.password
        });

        console.log(smtpLogin);
    } else {
        const user = await invoke("make_user", {username: "hello", password: "world!"});
        console.log(user);

        await dataStore.set('userdata', {
            username: user.username,
            password: user.password,
            extra: user.nonce
        })
    }

    setInterval(tick, 1000);
});
