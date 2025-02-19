<script setup>
import MetadataTableRowBigContent from './MetadataTableRowBigContent.vue';

    const {metadata} = defineProps(['metadata'])

    function format_date(date) {
        let year = date.substring(0, 4)
        let month = date.substring(4, 6)
        let day = date.substring(6, 7)
        return `${year}-${month}-${day}`
    }
</script>

<template>
    <template v-if="metadata.image!=null || metadata.dicom_info!=null">
        <h1 class="text-h6 font-weight-bold">Image</h1>
        <MetadataTableRow v-if="metadata.image.dims !=null" title="Dims" :val="metadata.image.dims"/>
        <template v-if="metadata.dicom_info!=null">
            <MetadataTableRow v-if="metadata.dicom_info.study_date !=null" title="Study date" :val="format_date(metadata.dicom_info.study_date)"/>
            <MetadataTableRow v-if="metadata.dicom_info.modality !=null"title="Modality" :val="metadata.dicom_info.modality"/>
            <MetadataTableRow v-if="metadata.dicom_info.body_part_examined !=null"title="Body part" :val="metadata.dicom_info.body_part_examined"/>
            <MetadataTableRowBigContent v-if="metadata.dicom_info.study_description !=null" title="Study description" :val="metadata.dicom_info.study_description"/>
            <MetadataTableRowBigContent v-if="metadata.dicom_info.series_description !=null"title="Series description" :val="metadata.dicom_info.series_description"/>
        </template>
    </template>
    <template v-if="metadata.dicom_info!=null">
        <h1 class="text-h6 font-weight-bold" style="padding-top: 30px;">Patient</h1>
        <MetadataTableRow title="Id" :val="metadata.dicom_info.patient_id"/>
        <MetadataTableRow title="Name" :val="metadata.dicom_info.patient_name"/>
        <MetadataTableRow title="Birthdate" :val="metadata.dicom_info.birth_date"/>
        <MetadataTableRow title="Sex" :val="metadata.dicom_info.sex"/>
    </template>
</template>

<style scoped>
    .main_row {
        border-width: 2px;
        border-color: red;
        height: 25px;
    }
</style>