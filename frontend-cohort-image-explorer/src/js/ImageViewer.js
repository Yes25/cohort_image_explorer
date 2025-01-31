export function rotate_right(input_image_class) {
    switch(input_image_class) {
        case "image_rotate_0":
            return "image_rotate_90";
        case "image_rotate_90":
            return "image_rotate_180";
        case "image_rotate_180":
            return "image_rotate_270";
        case "image_rotate_270":
            return"image_rotate_0";
        default:
            return "image_rotate_0";
    }
}

export function rotate_left(input_image_class) {
    switch(input_image_class) {
        case "image_rotate_0":
            return "image_rotate_270";
        case "image_rotate_90":
            return "image_rotate_0";
        case "image_rotate_180":
            return "image_rotate_90";
        case "image_rotate_270":
            return "image_rotate_180";
        default:
            return "image_rotate_0";
    }

}