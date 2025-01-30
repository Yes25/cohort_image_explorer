<script setup>
    const login = defineModel()
    defineEmits(['login'])

    const is_active = ref(false)
    
</script>

<template>
    <v-btn :class="login.acc_btn_class" icon="mdi-account" @click="is_active = !is_active"></v-btn>

    <v-dialog max-width="600" v-model="is_active">
        <v-card title="Login">
            <v-card-text>
                Plaese enter your s3 credentials.
            </v-card-text>
            <v-text-field class="cred_input_fields" v-model="login.credentials.username" label="Username"></v-text-field>
            <v-text-field class="cred_input_fields" type="password" v-model="login.credentials.password" label="Password"></v-text-field>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn text="Login" @click="$emit('login'); is_active = false"></v-btn>
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