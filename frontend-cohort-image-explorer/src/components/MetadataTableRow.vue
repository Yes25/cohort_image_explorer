<script setup>
    const {title, val} = defineProps(['title', 'val'])
</script>

<template>
    <v-row class="main_row">
        <v-col>
            <v-label>{{ title }}</v-label>
        </v-col>
        <v-col>
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
</style>