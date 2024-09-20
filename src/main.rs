use std::alloc::{alloc, dealloc, Layout};
use std::fmt::Display;

struct CelulaSimples<T> {
    conteudo: T,
    apontador: *mut CelulaSimples<T>,
}

struct ListaEncadeada<T> {
    n: u32,
    cabeca: *mut CelulaSimples<T>,
    ponta: *mut CelulaSimples<T>,
    // tamanho_unidade: usize,
}

impl<T> ListaEncadeada<T> {
    fn novo() ->Self {

        let layout:Layout = Layout::new::<CelulaSimples<T>>();

        let cabeca: *mut CelulaSimples<T> = unsafe {alloc(layout)} as *mut CelulaSimples<T>;
        ListaEncadeada {
            n: 0,
            cabeca: cabeca,
            ponta: cabeca.clone(),
        }
    }
    fn colocar(self: &mut Self,elemento: T) {
        let nova_celula: CelulaSimples<T> = CelulaSimples {
            conteudo: elemento,
            apontador: self.cabeca,
        };
        /*if self.n==0 {
            unsafe {self.cabeca.write(nova_celula)};
            self.n+=1;

            return
        }*/

        if self.n==0 {
            unsafe {self.cabeca.write(nova_celula)};
            self.n+=1;
        }
        else {
            let layout:Layout = Layout::new::<CelulaSimples<T>>();
            let ponteiro: *mut CelulaSimples<T>  = unsafe {alloc(layout) as *mut CelulaSimples<T>};
            let mut penultima_celula = unsafe { self.ponta.read() };
            penultima_celula.apontador = ponteiro;
            unsafe { self.ponta.write(penultima_celula) };
            unsafe { ponteiro.write(nova_celula) };
            self.ponta = ponteiro;
            self.n += 1;
        }

    }
    fn inserir_apos(self:&mut Self, endereco: *mut CelulaSimples<T>, conteudo: T) {
        //Insere o caractere 'conteudo' na celula imediatamente apos a celula que esta em 'endereco'
        //Identificando a celula atual (que sera a anterior à nova):
        let mut celula_anterior:CelulaSimples<T> = unsafe {endereco.read()};
        let celula_nova: CelulaSimples<T> = CelulaSimples {
            conteudo: conteudo,
            apontador: celula_anterior.apontador,
        };
        let layout:Layout = Layout::new::<CelulaSimples<T>>();
        unsafe { //modificando o apontador da celula anterior para apontar para a celula inserida
            let apontador_novo:*mut CelulaSimples<T> =alloc(layout) as *mut CelulaSimples<T>;
            apontador_novo.write(celula_nova);
            celula_anterior.apontador = apontador_novo;
            endereco.write(celula_anterior)
        };

    }
    fn proximo(self:&mut Self, endereco: *mut CelulaSimples<T>) -> (T,*mut CelulaSimples<T>) {
        if endereco==self.ponta {
            panic!()
        }
        unsafe {
            let celula_atual: CelulaSimples<T> =  {endereco.read()};
            let end_prox:*mut CelulaSimples<T> = celula_atual.apontador;
            let proxima_celula: CelulaSimples<T> =  {end_prox.read()};
             (proxima_celula.conteudo, (end_prox))
        }
    }
    fn proximo2(self:&Self, endereco: *const CelulaSimples<T>) -> (T,*const CelulaSimples<T>) {
        if endereco==self.ponta {
            panic!()
        }
        unsafe {
            let celula_atual: CelulaSimples<T> =  {endereco.read()};
            let end_prox:*const CelulaSimples<T> = celula_atual.apontador;
            let proxima_celula: CelulaSimples<T> =  {end_prox.read()};
            (proxima_celula.conteudo, (end_prox))
        }
    }
    fn ler_cabeca(self: &Self) ->(T,*const CelulaSimples<T>) {
        let celula: CelulaSimples<T> = unsafe {self.cabeca.read()};
        let conteudo: T = celula.conteudo;
        let apontador: *const CelulaSimples<T> = celula.apontador;
        (conteudo, apontador)

    }
    fn alterar(self: &Self, endereco: *mut CelulaSimples<T>, conteudo: T) {
        let mut celula = unsafe {endereco.read()};
        celula.conteudo=conteudo;
        unsafe {endereco.write(celula)};
    }
    fn deletar_apos(self: &mut Self, endereco: *mut CelulaSimples<T>) {
        assert!(endereco!=self.ponta);
        //Deleta a celula seguinte àquela do endereço fornecido
        let mut celula_anterior: CelulaSimples<T> = unsafe {endereco.read()};
        let ponteiro_remover:*mut CelulaSimples<T> = celula_anterior.apontador;
        let celula_a_remover: CelulaSimples<T> = unsafe {ponteiro_remover.read()};
        //Alterando o apontador da célula anterior para "pular" a célula deletada
        celula_anterior.apontador = celula_a_remover.apontador.clone();
        unsafe { endereco.write(celula_anterior) };
        self.n-=1;
        //Se a célula removida for a ponta da lista, a célula anterior vira a nova ponta:
        if ponteiro_remover==self.ponta {self.ponta = endereco}
        //Desalocando a memoria:
        let layout_remover: Layout = Layout::new::<CelulaSimples<T>>();
        unsafe { dealloc(ponteiro_remover as *mut u8, layout_remover) };
    }
    fn deletar_cabeca(self: &mut Self) {
        assert!(self.n>0);
        let cabeca_atual=self.cabeca;
        let celula_cabeca = unsafe {cabeca_atual.read()};
        self.cabeca = celula_cabeca.apontador.clone();
        let layout_remover: Layout = Layout::new::<CelulaSimples<T>>();
        unsafe { dealloc(cabeca_atual as *mut u8, layout_remover) };
        self.n-=1;
    }


}
impl<T:Display> ListaEncadeada<T> {
    fn imprimir_lista(self: &Self) {
        for s in self.into_iter() {
            print!("{}",s);
        }
        print!("\n");
    }
}
//todo()implementar lista dupla
struct IteradorLista<'a, T> {
    lista: &'a ListaEncadeada<T>,
    endereco_atual: *const CelulaSimples<T>,
    contagem: u32,
}
impl<'b,T> Iterator for IteradorLista<'b,T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.contagem==self.lista.n {None}
        else if self.contagem==0 {
            let (conteudo, _proximo_endereco) : (T, *const CelulaSimples<T>) = self.lista.ler_cabeca();
            // self.endereco_atual=proximo_endereco;
            self.contagem+=1;
            Some(conteudo)
        }
        else {
            let (conteudo, proximo_endereco): (T, *const CelulaSimples<T>) = self.lista.proximo2(self.endereco_atual);
            self.endereco_atual=proximo_endereco;
            self.contagem+=1;
            Some(conteudo)
        }
    }
}

impl<'a,T> IntoIterator for &'a ListaEncadeada<T> where T:'a {
    //Implementando um iterador para poder usar loops do tipo for com a lista encadeada
    type Item = T;

    type IntoIter = IteradorLista<'a,T> where T:'a;

    fn into_iter(self) -> Self::IntoIter {
        IteradorLista {
            lista: &self,
            endereco_atual: self.cabeca.clone(),
            contagem: 0,
        }
    }
}




fn teste_bom_dia() {
    let mut lista:ListaEncadeada<char>=ListaEncadeada::novo();
    println!("Início");
    let mensagem: &str ="Bom dia!";
    for letra in mensagem.chars() {
        lista.colocar(letra);
        println!("{}",letra);
    }
    println!("Escrevi\nLendo:");
    let mut endereco: *mut CelulaSimples<char> = lista.cabeca;
    let mut conteudo: char;
    let mut pos_inserir: *mut CelulaSimples<char> = lista.cabeca;
    unsafe { conteudo = lista.cabeca.read().conteudo; }
    print!("{}",conteudo);
    for _i in 1..8 {
       ( conteudo, endereco) = lista.proximo(endereco);
        print!("{}",conteudo);
        if conteudo=='m' { pos_inserir =endereco }
    }
    endereco=lista.cabeca;
    lista.inserir_apos(pos_inserir, 's');
    println!("\nTerminei de inserir");
    lista.imprimir_lista();
    println!("Inserindo exclamação!");
    let (_c, pos_apos) = lista.proximo(pos_inserir);
    lista.alterar(pos_apos, '!');
    lista.imprimir_lista();
    println!("Removendo exclamação!");
    lista.deletar_apos(pos_inserir);
    lista.imprimir_lista();
    println!("Removendo a primeira palavra");
    for _i in 1..=4 {
        lista.deletar_cabeca();
    }
    lista.imprimir_lista();
}

fn teste_numerico() {
    println!("Iniciando teste com dados numéricos e iterador");
    let numeros: [i32; 5] = [10, 20, 30, 40, 50];
    let mut lista: ListaEncadeada<i32> = ListaEncadeada::novo();
    for n in numeros {
        lista.colocar(n) }
    for elem in lista.into_iter() {
        println!("{}",elem);
    }
}
fn main() {
    teste_bom_dia();
    teste_numerico();
}