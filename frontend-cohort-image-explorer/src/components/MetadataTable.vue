<script setup>
    const {metadata} = defineProps(['metadata'])
</script>

<template>
    <template v-if="metadata.image!=null">
        <h1 class="text-h6 font-weight-bold">Image</h1>
        <MetadataTableRow title="Dims" :val="metadata.image.dims"/>
    </template>
    <template v-if="metadata.patient!=null">
        <h1 class="text-h6 font-weight-bold" style="padding-top: 30px;">Patient</h1>
        <MetadataTableRow title="Id:" :val="metadata.patient.id"/>
        <MetadataTableRow title="Name:" :val="metadata.patient.name"/>
        <MetadataTableRow title="Birthdate:" :val="metadata.patient.birth_date"/>
        <MetadataTableRow title="Sex:" :val="metadata.patient.sex"/>
    </template>
</template>

<style scoped>
    .main_row {
        border-width: 2px;
        border-color: red;
        height: 25px;
    }
</style>