<script setup>
import { rotate_left, rotate_right } from "@/js/ImageViewer";
import { get_auth_header, get_approved_images } from "@/js/helper_funcs";
import { ref } from "vue";

const api_url = "http://localhost:3030/api/";

const login = ref({
  username: "",
  password: "",
  all_buckets: [],
});

const image_slices = ref([]);
const curr_slice_idx = ref(0);
const num_slices = ref(0);
const image_rotation = ref("image_rotate_0");
const metadata = ref(null);

const bucket_name = ref("");
const bucket_content = ref([]);
const idx_img_curr_shown = ref(null);

const image_cache = ref(new Map());

const select_all = ref("false")
watch(
  () => select_all.value,
  (toggle_val) => {
    for(let item of bucket_content.value) {
      item.isSelected = toggle_val
    }
  },
);

watch(
  () => bucket_name.value,
  (new_value) => {
    fetchBucketContent(new_value);
  },
);

async function fetchImage(file_name) {
  const url = api_url + "bucket/" + bucket_name.value + "/image/" + file_name;
  try {
    const response = await fetch(url, {
      headers: get_auth_header(login.value.username, login.value.password),
    });
    if (!response.ok) {
      throw new Error(`Response status: ${response.status}`);
    }
    const json = await response.json();

    image_slices.value = json.slices;
    num_slices.value = json.slices.length;
    metadata.value = json.metadata;
  } catch (error) {
    console.error(error.message);
  }
}

async function fetchBucketContent(bucket_name) {
  const url = api_url + "bucket/" + bucket_name;
  try {
    const response = await fetch(url, {
      headers: get_auth_header(login.value.username, login.value.password),
    });
    if (!response.ok) {
      throw new Error(`Response status: ${response.status}`);
    }

    const json = await response.json();
    
    for(let item of json.bucket_contents) {
      bucket_content.value.push({"file_name": item, "isSelected": "false"})
    }
  } catch (error) {
    console.error(error.message);
  }
}



async function approve() {
  const url = api_url + "approve/bucket/" + bucket_name.value
  let headers = get_auth_header(login.value.username, login.value.password)
  headers["Content-Type"] = "application/json"
  let body = JSON.stringify({
          "username": login.value.username,
          "approved_imges": get_approved_images(bucket_content.value)
        })
  try {
    const response = await fetch(url, {
      method: "POST",
      headers: headers,
      body: body
    });
    if (!response.ok) {
      throw new Error(`Response status: ${response.status}`);
    }

  } catch (error) {
    console.error(error.message);
  }
}
</script>

<template>
  <v-container class="main_container w-100">
    <v-row align="center" justify="center">
      <v-spacer />
      <h1 class="text-h3 font-weight-bold">Cohort Explorer</h1>
      <v-spacer />
      <LoginDialog v-model="login" />
    </v-row>
    <v-row>
      <v-col class="main_col meta_col" cols="3">
        <h1 class="metadata_title text-h5 font-weight-bold">Meta data</h1>
        <MetadataTable v-if="metadata!=null" :metadata="metadata"/>
      </v-col>
      <v-col class="main_col">
        <v-row align="center" justify="center">
          <v-btn
            class="rotation_btn"
            density="compact"
            icon="mdi-file-rotate-left-outline"
            @click="image_rotation = rotate_left(image_rotation)"
          />
          <v-btn
            class="rotation_btn"
            density="compact"
            icon="mdi-file-rotate-right-outline"
            @click="image_rotation = rotate_right(image_rotation)"
          />
        </v-row>
        <!-- v-responsive needs to be here because otherwise it seems that we can't rotate the image in every aspect ratio -->
        <v-responsive>
          <v-img
            v-if="image_slices.length > 0"
            :id="image_rotation"
            class="image_class"
            v-bind:src="
              'data:image/jpeg;base64,' + image_slices[curr_slice_idx]
            "
          />
          <v-sheet
            v-else
            class="image_class d-flex align-center justify-center flex-wrap text-center mx-auto px-4"
            :height="400"
            :width="400"
            rounded
            align="center"
            justify="center"
          >
            <h1 class="text-h3 font-weight-bold">No Image</h1>
          </v-sheet>
        </v-responsive>
        <v-slider
          v-model="curr_slice_idx"
          :max="num_slices - 1"
          :step="1"
          style="padding: 20px"
        />
      </v-col>
      <v-col class="main_col">
        <v-select
          label="Bucket"
          :items="login.all_buckets"
          v-model="bucket_name"
          @input="fetchBucketContent(bucket_name)"
        />
        <!-- The :items is just here to somehow show one instance of the inner nodes <- hacky... there must be a better way -->
        <v-virtual-scroll class="image-list" :items="[1]">
          <v-list>
            <v-list-item v-for="(item, idx) in bucket_content"
              :key="idx"
              :title="item.file_name"
              :value="item.file_name"
              @click="
                fetchImage(item.file_name);
                idx_img_curr_shown = idx;
              "
            >
              <template #prepend>
                <v-list-item-action start>
                  <v-checkbox-btn true-value="true" false-value="false" v-model="item.isSelected"></v-checkbox-btn>
                </v-list-item-action>
              </template>
            </v-list-item>
          </v-list>
        </v-virtual-scroll>
        <v-row>
          <v-btn class="approve_btn" @click="approve()">Approve selected</v-btn>
          <v-checkbox class="toggle_all" label="toggle all" true-value="true" false-value="false" v-model="select_all"></v-checkbox>
      </v-row>
      </v-col>
    </v-row>
  </v-container>
</template>

<style scoped>
.main_container {
  height: 100vh;
  width: 100vb;
  /* background-color: blue; */
}

.image-list {
  background-color: #262424;
  border-radius: 5px;
  max-height: 65vh;
}

#image_rotate_90 {
  transform: rotate(90deg);
}

#image_rotate_180 {
  transform: rotate(180deg);
}

#image_rotate_270 {
  transform: rotate(270deg);
}

#image_rotate_0 {
  transform: rotate(0deg);
}

.rotation_btn {
  margin-top: 50px;
}

.account_btn {
  margin: 32px;
}

.image_class {
  margin-top: 25px;
  margin-bottom: 5px;
  max-height: 400px;
  padding: 2px;
}

.main_cols {
  height: 80vh;
}

.metadata_title {
  margin-top: 70px;
}

.approve_btn {
  margin-top: 20px;
  margin-left: 12px;
}

.toggle_all {
  margin-top: 10px;
}
</style>
