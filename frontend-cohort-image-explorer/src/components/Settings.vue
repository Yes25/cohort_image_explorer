<script setup>
    const {settings} = defineProps(['settings'])

    const is_active = ref(false)

    const init_num_to_prefetch = ref(settings.num_to_prefetch)
    const init_max_cached_imgs = ref(settings.num_max_cached_imgs)
</script>

<template>
    <v-btn icon="mdi-wrench" @click="is_active = !is_active"></v-btn>

    <v-dialog max-width="600" min-height="200" v-model="is_active" @keydown.enter="try_login()">
        <v-card title="Settgings">
            <v-number-input
                class="input_fields"
                v-model="init_num_to_prefetch"
                label="Number of Images to prefetch"
                :max="30"
                :min="0"
            ></v-number-input>
            <v-number-input
                class="input_fields"
                v-model="init_max_cached_imgs"
                label="Number of max cached images"
                :max="30"
                :min="0"
            ></v-number-input>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn text="Cancel" @click="is_active = !is_active"></v-btn>
                <v-btn text="Save" @click="is_active = !is_active; $emit('updateSettings', init_num_to_prefetch, init_max_cached_imgs)"></v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<style scoped>
    .account_btn {
        margin: 32px;
    }

    .input_fields {
        padding-left: 20px;
        padding-right: 20px;
    }
</style>