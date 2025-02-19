<script setup>
    const {title, val} = defineProps(['title', 'val'])
</script>

<template>
    <v-row class="main_row top_row">
        <v-col>
            <v-label class="title">{{ title }}</v-label>
        </v-col>
    </v-row>
    <v-row>
        <v-col class="main_row">
            <v-label>{{ val }}</v-label>
        </v-col>
    </v-row>
</template>

<style scoped>
    .main_row {
        border-width: 2px;
        border-color: red;
        height: 25px;
    }
    .top_row {
        margin-top: 25px;
    }

    .title {
        font-weight: bold;
    }
</style>