"use client";
import Prove from "./prove";
import Footer from "./footer";

export default function Home() {
    return (
        <>
            <main className="min-h-screen flex-col items-center justify-between p-10 break-words dark:text-slate-400 text-slate-500">
                <h1 className="text-4xl font-bold mb-8">
                    Mopro Website Prover Tests
                </h1>

                <Prove circuit="multiplier2"></Prove>
                <Prove circuit="keccak256"></Prove>
                <Prove circuit="RSA"></Prove>
                <div className="p-20"></div>
            </main>
            <Footer></Footer>
        </>
    );
}
