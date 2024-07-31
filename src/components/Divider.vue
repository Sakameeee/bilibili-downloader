<script setup lang="ts">
import {onMounted} from "vue";

interface Props {
  size: number;
}

const props = withDefaults(defineProps<Props>(), {
  size: 20,
});

onMounted(() => {
  document.getElementById("divider").style.height = `${props.size}px`;
})
</script>

<template>
<div id="divider"></div>
</template>

<style scoped>
#divider {
  //height: 15px;
}
</style>