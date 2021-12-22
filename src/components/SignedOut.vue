<template>
  <main class="container text-center">
    <h1 class="mt-5">Welcome to NEAR!</h1>
    <p>
      Welcome!
    </p>
    <p class="pt-3">
      <button class="btn btn-primary btn-lg" v-on:click="login">Sign in</button>
    </p>
  </main>
</template>

<script>
import {login} from "../utils"

export default {
  name: "SignedOut",
  methods: {
    login() {
      console.log("calling utils.login")
      login()
    },
  },
}
</script>
