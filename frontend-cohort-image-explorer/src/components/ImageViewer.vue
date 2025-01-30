<script setup>
    import { ref } from 'vue'
    
    const api_url = "http://localhost:3030/api/"

    const login = ref(
        {
            "credentials":
            {
                "username":"", 
                "password": ""
            },
            "acc_btn_class": "account_btn"
        }
        )


    const image_slices = ref([])
    const curr_img_idx = ref(0)
    const num_slices = ref(0)
    const bucket_content = ref([])
    const bucket_name = ref('ixi-test-bucket')

    const image_class = ref("image_rotate_0")

    async function fetchImage(file_name) {
        const url = api_url + "image/" + file_name;
        try {
            const response = await fetch(url);
            if (!response.ok) {
                throw new Error(`Response status: ${response.status}`);
            }

            const json = await response.json();
            
            image_slices.value = json.slices;
            num_slices.value = json.slices.length;
        } catch (error) {
            console.error(error.message);
        }
    }

    async function fetchBucktContent(bucket_name) {
        const url =  api_url + "bucket/" + bucket_name;
        try {
            const response = await fetch(url);
            if (!response.ok) {
                throw new Error(`Response status: ${response.status}`);
            }

            const json = await response.json();
            
            bucket_content.value = json.bucket_contents;
        } catch (error) {
            console.error(error.message);
        }
    }

    function rotate_left(input_image_class) {
        switch(input_image_class) {
            case "image_rotate_0":
                image_class.value = "image_rotate_270";
                break;
            case "image_rotate_90":
                image_class.value = "image_rotate_0";
                break;
            case "image_rotate_180":
                image_class.value = "image_rotate_90";
                break;
            case "image_rotate_270":
                image_class.value = "image_rotate_180";
                break;
            default:
                image_class.value = "image_rotate_0";
        }

    }

    function rotate_right(input_image_class) {
        switch(input_image_class) {
            case "image_rotate_0":
                image_class.value = "image_rotate_90";
                break;
            case "image_rotate_90":
                image_class.value = "image_rotate_180";
                break;
            case "image_rotate_180":
                image_class.value = "image_rotate_270";
                break;
            case "image_rotate_270":
                image_class.value = "image_rotate_0";
                break;
            default:
                image_class.value = "image_rotate_0";
        }
    }

    async function try_login() {
        const url =  api_url + "buckets";
        try {
            const response = await fetch(url, {
                headers: {
                    "Authorization": "Basic " +  Base64.encode(login.credentials.username+':'+login.credentials.password),
                }
            })
            if (!response.ok) {
                throw new Error(`Response status: ${response.status}`);
            }

            const json = await response.json();
            
            bucket_content.value = json.bucket_contents;
        } catch (error) {
            console.error(error.message);
        }

        login.value.acc_btn_class = "account_btn_valid"

    }

</script>

<template>
    <v-row>
        <v-spacer></v-spacer>
        <LoginDialog v-model="login" @login="try_login"></LoginDialog>
    </v-row>
    <v-row align="center" justify="center">
        <h1 class="text-h3 font-weight-bold">Cohort Explorer</h1>
    </v-row>
    <v-container class="main_container">
        <v-responsive class="align-centerfill-height mx-auto" max-width="900">
            <v-row>
                <v-col>
                    <v-row>
                        <v-text-field v-model="bucket_name" label="Bucket name" style="padding: 20px;"></v-text-field>
                        <v-btn @click=fetchBucktContent(bucket_name) style="margin: 20px; margin-top: 30px;">Fetch Images</v-btn>
                        <v-btn class="rotaion_btn" density="compact" icon="mdi-file-rotate-left-outline" @click="rotate_left(image_class)"></v-btn>
                        <v-btn class="rotaion_btn" density="compact" icon="mdi-file-rotate-right-outline" @click="rotate_right(image_class)"></v-btn>
                    </v-row>
                    <v-img :class="image_class" height="400" v-bind:src="'data:image/jpeg;base64,' + image_slices[curr_img_idx]" />
                    <v-slider v-model="curr_img_idx" :max="num_slices-1" :step="1" style="padding: 20px;"></v-slider>
                </v-col>
                <v-col>
                    <v-virtual-scroll class="image-list"
                        :max-height="600" 
                        :items="bucket_content"
                    >
                        <template v-slot:default="{ item }">
                            <v-list-item
                                :title="item" 
                                :value="item" 
                                @click="fetchImage(item)">
                            </v-list-item>
                        </template>   
                    </v-virtual-scroll>
                </v-col>
            </v-row>
        </v-responsive>
    </v-container>
</template>


<style scoped>
    .main_container {
        margin-top: 50px;
    }

    .image-list {
        background-color: #262424;
        margin-top: 50px;
        border-radius: 5px;
    }

    .image_rotate_90 {
        transform:rotate(90deg);
    }

    .image_rotate_180 {
        transform:rotate(180deg);
    }

    .image_rotate_270 {
        transform:rotate(270deg);
    }

    .image_rotate_0 {
        transform:rotate(0deg);
    }

    .rotaion_btn {
        margin-top: 32px;
    }

    .account_btn {
        margin: 32px;
    }
</style>