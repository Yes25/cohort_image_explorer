export function get_auth_header(username, password) {
    const creds = btoa(username + ':' + password)
    return {"Authorization": "Basic " + creds}
}

export function get_approved_images(images) {
    let approved_list = []
    for(let item of images) {
        if(item.isSelected == "true") {
            approved_list.push(item.file_name)
        }
    }
    return approved_list
}
