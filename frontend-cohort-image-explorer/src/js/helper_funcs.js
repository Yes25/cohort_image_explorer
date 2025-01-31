export function get_auth_header(username, password) {
    const creds = btoa(username + ':' + password)
    return {"Authorization": "Basic " + creds}
}