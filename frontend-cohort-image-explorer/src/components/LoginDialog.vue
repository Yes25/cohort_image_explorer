<script setup>
    import { get_auth_header } from '@/js/helper_funcs'

    const login = defineModel()
    // defineEmits(['login'])

    const is_active = ref(true)
    const acc_btn_class = ref("account_btn")
    
    async function try_login() {
        const url =  'http://localhost:3030/api/' + "buckets";
        try {
            const response = await fetch(url, { headers: get_auth_header(login.value.username, login.value.password) })
            if (!response.ok) {
                throw new Error(`Response status: ${response.status}`);
            }
            const bucket_list = await response.json();
            acc_btn_class.value = "account_btn_valid";
            login.value.all_buckets = bucket_list;
            is_active.value = false;
        } catch (error) {
            console.error(error.message);
        }
    }

</script>

<template>
    <v-btn :class="acc_btn_class" icon="mdi-account" @click="is_active = !is_active"></v-btn>

    <v-dialog max-width="600" v-model="is_active" @keydown.enter="try_login()">
        <v-card title="Login">
            <v-card-text>
                Plaese enter your s3 credentials.
            </v-card-text>
            <v-text-field class="cred_input_fields" v-model="login.username" label="Username"></v-text-field>
            <v-text-field class="cred_input_fields" type="password" v-model="login.password" label="Password"></v-text-field>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn text="Login" @click="try_login()"></v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<style scoped>
    .account_btn {
        margin: 32px;
    }

    .account_btn_valid {
        margin: 32px;
        background-color: mediumSeaGreen;
    }

    .cred_input_fields {
        padding-left: 20px;
        padding-right: 20px;
    }
</style>