# Projeto Warehouse

---

## Regras de negocio
> Notas

* ### Todas as regras devem registrar o usuario que realizou a requisiçao.

<br>

1. ### Cria almoxarifado
Deve criar uma nova instancia de almoxarifado e retornar a instancia como resposta, ou erro.

```json
{
  "id": "id do almoxarifado",
  "name": "nome do almoxarifado",
  "tools": [], // lista de ferramentas presentes no almoxarifado
  "assets": [] // lista de patrimonios presentes no almoxarifado
}
 ```

2. ### Consulta almoxarifado
Deve retornar o almoxarifado que corresponde ao nome na requisiçao.

3. ### Cria ferramenta