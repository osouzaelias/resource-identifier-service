import http from 'k6/http';
import { check, sleep } from 'k6';
import { htmlReport } from "https://raw.githubusercontent.com/benc-uk/k6-reporter/main/dist/bundle.js";

export let options = {
    stages: [
        { duration: '30s', target: 20 }, // abaixo de carga normal
        { duration: '1m', target: 100 }, // carga média
        { duration: '30s', target: 200 }, // pico de carga
        { duration: '2m', target: 0 },   // escala abaixo para 0 usuários
    ],
};

export default function () {
    let res = http.post('http://localhost:3030/format', JSON.stringify({
        legal_entity: "youpay",
        tenant: "partner",
        segment: "business",
        payment_instrument: "pix",
        customer_id: "123456789"
    }), {
        headers: { 'Content-Type': 'application/json' },
    });

    check(res, { 'status was 200': (r) => r.status == 200 });
    sleep(1);
}

export function handleSummary(data) {
    return {
        "summary.html": htmlReport(data),
    };
}
