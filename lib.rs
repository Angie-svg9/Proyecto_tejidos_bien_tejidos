use anchor_lang::prelude::*;
declare_id!("BavuxHPkdt66ArWDT93jezxBqzvErwqw2DmfZZoJZY7B");

#[program] 
pub mod tienda_crochet {
    use super::*;


    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {
       
        let comprador_id = context.accounts.comprador.key();
        msg!("Comprador_id: {}", comprador_id);

        let hilos: Vec<Hilo> = Vec::new(); 

        
        context.accounts.tienda.set_inner(Tienda { 
            comprador: comprador_id,
            nombre,
            hilos,
            
        });
        Ok(()) 
    }


    pub fn agregar_hilo(context: Context<NuevoHilo>, nombre: String, color: String, grosor: u16) -> Result<()> {
        require!( 
            context.accounts.tienda.comprador == context.accounts.comprador.key(), 
            Errores::NoEresElComprador 
        ); 

        let nuevo_hilo = Hilo { 
            nombre,
            color,
            grosor,
            disponible: true,
        };

        context.accounts.tienda.hilos.push(nuevo_hilo); 

        Ok(()) 
    }

    
    pub fn eliminar_hilo(context: Context<NuevoHilo>, nombre: String) -> Result<()> {
        require!(
            context.accounts.tienda.comprador == context.accounts.comprador.key(),
            Errores::NoEresElComprador
        );

        let hilos = &mut context.accounts.tienda.hilos;
        for i in 0..hilos.len() { // Se itera mediante el indice todo el contenido del vector en busca del libro a eliminar
            if hilos[i].nombre == nombre { // Si lo encuentra prodece a borrarlo mediante el metodo remove
                hilos.remove(i);
                msg!("Hilo {} eliminado!", nombre); // Mensaje de borrado exitoso
                return Ok(()); // Transaccion exitosa
            }
        }
        Err(Errores::HiloNoExiste.into()) // Transaccion fallida, nunca encontro el libro
    }

   
    pub fn ver_hilos(context: Context<NuevoHilo>) -> Result<()> {
        require!( // Medida de seguridad 
            context.accounts.tienda.comprador == context.accounts.comprador.key(),
            Errores::HiloNoExiste
        );

        // :#? requiere que NuevoLibro tenga atributo Debug. Permite la visualizacion completa del vector en el log
        msg!("La lista de hilos actualmente es: {:#?}", context.accounts.tienda.hilos); // Print en log
        Ok(()) // Transaccion exitosa 
    }

    
    //////////////////////////// Instruccion: Alternar Estado /////////////////////////////////////
    /* 
    Cambia el estado de disponible de false a true o de true a false.

    Parametros de entrada:
        * nombre -> Nombre del libro -> string
     */
    pub fn alternar_hilo(context: Context<NuevoHilo>, nombre: String) -> Result<()> {
        require!( // Medida de seguridad
            context.accounts.tienda.comprador == context.accounts.comprador.key(),
            Errores::NoEresElComprador
        );

        let hilos = &mut context.accounts.tienda.hilos; // Referencia mutable al vector de libros
        for i in 0..hilos.len() { // Se itera mediante el indice el vector de libros
            let estado = hilos[i].disponible;  // Se almacena el estado del vector actual

            if hilos[i].nombre == nombre { // Si ecuentra el nombre del libro procede a cambiar el valor del estado 
                let nuevo_estado = !estado;
                hilos[i].disponible = nuevo_estado;
                msg!("El hilo: {} ahora esta disponible: {}", nombre, nuevo_estado); // log print de la nueva disponibilidad
                return Ok(()); // Transaccion exitosa
            }
        }

        Err(Errores::HiloNoExiste.into()) // Transaccion fallida, libro no existe
    }

}

/*
Codigos de error
Todos los codigos se almacenan en un enum con la siguiente estructura:
#[msg("MENSAJE DE ERROR")] (dentro de las comillas)
NombreDelError, (En camel case)
*/
#[error_code]
pub enum Errores {
    #[msg("Error, no eres el comprador del hilo que deseas modificar")]
    NoEresElComprador,
    #[msg("Error, el hilo con el que deseas interactuar no existe")]
    HiloNoExiste,
}

#[account] // Especifica que el strcut es una cuenta que se almacenara en la blockchain
#[derive(InitSpace)] // Genera la constante INIT_SPACE y determina el espacio de almacenamiento necesario 
pub struct Tienda { // Define la Biblioteca
    pub comprador: Pubkey, // Pubkey es un formato de llave publica de 32 bytes 

    #[max_len(60)] // Cantidad maxima de caracteres del string: nombre
   pub  nombre: String,

    #[max_len(10)] // Tamaño maximo del vector libros 
   pub  hilos: Vec<Hilo>,
}

/*
Struct interno o secundario (No es una cuenta). Se define por derive y cuenta con los siguientes atributos:
    * AnchorSerialize -> Permite guardar el struct en la cuenta 
    * AnchorDeserialize -> Permite leer su contenido desde la cuenta 
    * Clone -> Para copiar su contenido o valores 
    * InitSpace -> Calcula el tamaño necesario para ser almacenado en la blockchain
    * PartialEq -> Para usar sus valores y compararlos con "=="
    * Debug -> Para mostrarlo en log con ":?" o ":#?"
*/
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Hilo {
   #[max_len(60)]
    pub nombre: String,
    #[max_len(30)] 
    pub color: String, 
    pub grosor: u16, 
    pub disponible: bool,
}


// Creacion de los contextos para las instrucciones (funciones)
#[derive(Accounts)] // Especifica que este struct describe las cuentas que se requieren para determinada instruccion
pub struct NuevaTienda<'info> { // contexto de la instruccion
    #[account(mut)] 
    pub comprador: Signer<'info>, // Se define que el owner como el que pagara la transaccion, por eso es mut, para que cambie el balance de la cuenta

    #[account(
        init, // Inidica que al llamar la instruccuion se creara una cuenta
        // puede ser remplazado por "init_if_needed" para que solo se cree una vez por caller
        payer = comprador, // Se especifica que quien paga el llamado a la instruccion, en este caso llama la instruccion 
        space = Tienda::INIT_SPACE + 8, // Se calcula el espacio requerido para almacenar el Solana Program On-Chain
        seeds = [b"tienda", comprador.key().as_ref()], // Se especifica que la cuenta es una PDA que depende de un string y el id del owner
        bump // Metodo para determinar el el id de la biblioteca en base a lo anterior 
    )]
    pub tienda: Account<'info, Tienda>, // Se especifica que la cuenta creada (PDA) almacenara la biblioteca 

    pub system_program: Program<'info, System>, // Programa necesario para crear la cuenta 
}

// Contexto para la creacion y modificacion de libros 
#[derive(Accounts)] // Especifica que este struct se requiere para todas las instrucciones relacionadas con la creacion o modificacion de Libro
pub struct NuevoHilo<'info> {
    pub comprador: Signer<'info>, // El owner de la cuenta es quien paga la transaccion

    #[account(mut)] 
    pub tienda: Account<'info, Tienda>, // Se marca biblioteca como mutable porque se modificara tanto el vector como los libros que contiene
}
