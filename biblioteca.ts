import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
// 1. ASEGÚRATE de que este nombre coincida con el de tu archivo en target/types/
import { Tiendapeliculas } from "../target/types/tiendapeliculas"; 
import { web3 } from "@coral-xyz/anchor";
import { expect } from "chai";

describe("tiendapeliculas", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // 2. Si esto está en rojo, corre 'anchor build' en la terminal
  const program = anchor.workspace.Tiendapeliculas as Program<Tiendapeliculas>;

  const [tiendaPda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("tienda"),
      provider.wallet.publicKey.toBuffer(),
    ],
    program.programId
  );

  it("Inicializa la tienda y agrega una película", async () => {
    
    // 3. Los métodos en TS SIEMPRE empiezan con minúscula (camelCase)
    // Aunque en Rust sean crear_tienda, aquí son crearTienda
    try {
      await program.methods
        .crearTienda("Cine de Barrio")
        .accounts({
          owner: provider.wallet.publicKey,
          // @ts-ignore -> Si 'tienda' sale en rojo por el tipo de PDA
          tienda: tiendaPda,
          systemProgram: web3.SystemProgram.programId,
        })
        .rpc();

      const precioSol = new anchor.BN(1000000000); // 1 SOL
      const precioUsdc = new anchor.BN(15000000); // 15 USDC

      await program.methods
        .agregarPelicula(
          "El Padrino",
          "Crimen y Mafia",
          new anchor.BN(1972),
          "Francis Ford Coppola",
          "Drama",
          precioSol,
          precioUsdc
        )
        .accounts({
          tienda: tiendaPda,
          owner: provider.wallet.publicKey,
        })
        .rpc();

      // 4. Leer la cuenta: El nombre aquí debe ser igual al struct de Rust en minúsculas
      const tiendaAccount = await program.account.tiendapeliculas.fetch(tiendaPda);
      
      console.log("Tienda:", tiendaAccount.nombre);
      console.log("Películas:", tiendaAccount.peliculas);

      expect(tiendaAccount.nombre).to.equal("Cine de Barrio");
      
    } catch (error) {
      console.error("Error detectado:", error);
      throw error;
    }
  });
});
