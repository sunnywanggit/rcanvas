<script setup>
import "../../rlib/pkg/rlib_bg.wasm?init"
import init, { test, unred } from '../../rlib/pkg/rlib.js';
import { onMounted } from "@vue/runtime-core";

const test1 = async () =>  {
 await init()
 console.log(test());
 const start =  Date.now();
 const readResult = await unred('myCanvas');
 console.log(readResult);
 console.log('time wasm', Date.now() - start)
}

onMounted(()=> {
  const start =  Date.now();
  const canvas = document.getElementById('myCanvas');
  console.log(canvas.toDataURL('image/png'));
  console.log('time JS', Date.now() - start)
})
</script>

<template>
  <div>
    <canvas id="myCanvas" width="9000" height="9000"/>
  </div>
</template>

<style scoped>
</style>
