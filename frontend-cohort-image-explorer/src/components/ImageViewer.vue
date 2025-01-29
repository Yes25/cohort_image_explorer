<script setup>
    import { ref } from 'vue'

    let image_slices = ref([])
    let curr_img_idx = ref(0)
    let num_slices = ref(0)
    let bucket_content = ref([])
    let bucket_name = ref('ixi-test-bucket')

    let filenames_loading = ref(true)

    async function fetchImage(file_name) {
        const url = "http://localhost:3030/api/image/" + file_name;
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
        filenames_loading.value = true;

        const url = "http://localhost:3030/api/bucket/" + bucket_name;
        try {
            const response = await fetch(url);
            if (!response.ok) {
                throw new Error(`Response status: ${response.status}`);
            }

            const json = await response.json();
            
            bucket_content.value = json.bucket_contents;
            filenames_loading.value = false;
        } catch (error) {
            console.error(error.message);
        }
    }


</script>

<template>
    <v-container class="fill-height">
        <v-responsive class="align-centerfill-height mx-auto" max-width="900">
            <div class="text-center">
                <h1 class="text-h3 font-weight-bold">Image Viewer</h1>
            </div>
            <v-row>
                <v-col>
                    <v-row>
                        <v-text-field v-model="bucket_name" label="Bucket name" style="padding: 20px;"></v-text-field>
                        <v-btn @click=fetchBucktContent(bucket_name) style="margin: 20px; margin-top: 30px;">Fetch Images</v-btn>
                    </v-row>
                    <v-img class="mb-4" height="400" v-bind:src="'data:image/jpeg;base64,' + image_slices[curr_img_idx]" />
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
    .image-list {
        background-color: #262424;
        margin-top: 50px;
        border-radius: 5px;
    }
</style>