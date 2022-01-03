<template>
  <main class="container">
    <div class="text-center mt-lg-5 mb-lg-2 mt-2">
      <img src="../assets/logo.png" alt="logo" width="200">
    </div>
    <h2 class="mb-lg-5 mb-3 text-center">
      NEAR Payment Vouchers
    </h2>
    <div class="landing-top">
      <img src="../assets/sample.jpg" alt="sample" class="sample-image">
      <div>
        <p class="mt-4 pt-3">
          <b>Pay offline using NEAR Payment Vouchers:</b>
        </p>
        <ol>
          <li>Login using your NEAR wallet.</li>
          <li>Generate payment vouchers (it will lock your funds).</li>
          <li>Send link or show QR-code to make a payment.</li>
        </ol>
        <p class="pt-3 text-md-start text-center">
          <button class="btn btn-primary" v-on:click="login">Sign In</button>
        </p>
      </div>
    </div>
    <footer>
      NEAR Voucher &copy; 2022
    </footer>
  </main>
</template>

<script>
import {login} from "../utils"

export default {
  name: "SignedOut",
  methods: {
    login() {
      login()
    },
  },
}
</script>

<style>
.landing-top {
  display: grid;
  grid-template-columns: 45% auto;
  grid-gap: 5%;
  font-size: 17px;
}

.landing-top img {
  max-width: 100%;
}

.btn {
  padding: 8px 28px !important;
}

.sample-image {
  border-radius: 15px;
  box-shadow: inset 0 0 10px rgba(0, 0, 0, 0.1);
  padding: 10px;
  background: #efefef;
}

footer {
  position: absolute;
  bottom: 10px;
  left: 0;
  right: 0;
  text-align: center;
  color: #999;
  font-size: 12px;
  border-top: 1px dashed #CCC;
  padding-top: 10px;
}

@media all and (max-width: 640px) {
  .landing-top {
    display: block;
  }
}
</style>