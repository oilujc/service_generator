---
name: agno-agent-creator
description: Asistente para la creación de agentes basados en la librería Agno siguiendo la arquitectura del proyecto accountable. Genera archivos de agente, modelos Pydantic y los registra en la factoría.
---

# Agno Agent Creator Skill

Este skill ayuda a generar agentes consistentes con el patrón de diseño del proyecto accountable usando FastAPI, MongoDB y la librería Agno.

## Arquitectura de Agents

### Estructura

```
app/agents/
├── agent_factory.py              # Registro y creación de agents
└── agents/
    ├── base_agent.py             # Clase abstracta base
    └── {nombre}_agent.py         # Implementación específica
```

### Principios Clave

1. **Separación de responsabilidades**:
   - **Agente**: Define comportamiento, instrucciones y esquemas de I/O
   - **Servicio**: Proporciona tools (funciones), orquesta ejecución, maneja repositorios
   - **Factory**: Crea instancias del agente con configuración dinámica

2. **Tools dinámicas**: Las tools (funciones) se pasan desde el servicio, NO se definen en el agente
3. **Sesiones en MongoDB**: La memoria del agente se persiste usando MongoDb de Agno
4. **BaseAgent**: Proporciona `_get_db()` para configurar persistencia de sesiones

## Flujo de Trabajo para Crear un Agente

### 1. Definición de Modelos (Input/Output)

Define los modelos Pydantic en `app/models/agent_model.py` o en un archivo dedicado `app/models/{agent}_model.py`.

```python
from pydantic import BaseModel
from typing import List, Optional

class AnalysisRequest(BaseModel):
    text: str
    context: Optional[dict] = None

class AnalysisResponse(BaseModel):
    result: str
    confidence: float
    suggestions: List[str]
```

### 2. Creación del Archivo del Agente

Genera el archivo en `app/agents/agents/{nombre}_agent.py`.

#### Template del Agente

```python
import os
from agno.agent import Agent
from agno.models.deepseek import DeepSeek  # o OpenAIChat, etc.
from textwrap import dedent

from agents.agents.base_agent import BaseAgent
from models.agent_model import AnalysisRequest, AnalysisResponse

class AnalysisAgent(BaseAgent):
    name = "AnalysisAgent"
    model = DeepSeek(id="deepseek-chat")

    def build(self, *args, **kwargs):
        """
        Construye el agente de Agno.
        
        NOTA: Las tools se pasan dinámicamente desde el servicio
        a través de kwargs.get("tools", []). No definas tools aquí.
        """
        return Agent(
            name=self.name,
            model=self.model,
            db=self._get_db("analysis_agent_sessions"),  # Para persistencia de sesiones
            description="Agente experto en análisis de texto.",
            instructions=dedent("""
                # ROLE
                Eres un analista experto en procesamiento de texto.

                # INSTRUCCIONES
                1. Analiza el texto proporcionado
                2. Identifica patrones clave
                3. Proporciona sugerencias accionables

                # REGLAS
                - Usa las herramientas disponibles para obtener contexto
                - Sé conciso pero completo
                - Idioma: Español
            """),
            tools=kwargs.get("tools", []),  # Tools inyectadas desde el servicio
            output_schema=AnalysisResponse,
            input_schema=AnalysisRequest,
            add_history_to_context=True,
            num_history_runs=5,
            markdown=True
        )
```

#### Cuándo usar `db` en el modelo

Usa `db=self._get_db("collection_name")` en el método `build()` cuando:
- Necesites persistir el historial de conversaciones del agente
- Quieras que el agente mantenga contexto entre llamadas
- Uses `add_history_to_context=True`

NO uses `db` cuando:
- El agente sea stateless (sin memoria entre llamadas)
- Cada consulta sea independiente
- No necesites historial de conversación

### 3. Registro en AgentFactory

Actualiza `app/agents/agent_factory.py`:

```python
from typing import Dict, Type
from agents.agents.base_agent import BaseAgent
from agents.agents.accounting_agent import AccountingAgent
from agents.agents.analysis_agent import AnalysisAgent  # Nuevo agente

class AgentFactory:
    def __init__(self):
        self.agents: Dict[str, Type[BaseAgent]] = {
            "accounting_agent": AccountingAgent(),
            "analysis_agent": AnalysisAgent()  # Registrar aquí
        }

    def get_agent(self, name: str, *args, **kwargs):
        agent = self.agents.get(name)
        if not agent:
            raise ValueError(f"Agent {name} not found")
        return agent.build(*args, **kwargs)
```

### 4. Uso en Servicios

Los servicios son responsables de:
- **Proporcionar tools**: Funciones que el agente puede usar
- **Inyectar dependencias**: Repositorios, configuración, etc.
- **Orquestar la ejecución**: Llamar al factory y ejecutar el agente

#### Servicio con Agente - Ejemplo Completo

```python
from typing import List, Dict, Any, Optional
from pymongo.database import Database
from fastapi import Depends
from agents.agent_factory import AgentFactory
from dependencies import get_db, get_agent_factory

from models.agent_model import AnalysisRequest, AnalysisResponse
from utils.response_exception import ResponseException
from database.repositories.document_repository import DocumentRepository

class AnalysisService:
    def __init__(self, db: Database, agent_factory: AgentFactory):
        self.agent_factory = agent_factory
        self.document_repo = DocumentRepository(db)

    def search_documents(self, query: str, limit: int = 10) -> List[Dict[str, Any]]:
        """
        Tool para el agente: Busca documentos relevantes.
        Las tools deben tener docstrings claros para el LLM.
        """
        documents = self.document_repo.search(query, limit)
        return [doc.model_dump() for doc in documents]

    def save_analysis(self, document_id: str, analysis: Dict[str, Any]) -> str:
        """
        Tool para el agente: Guarda el resultado del análisis.
        """
        self.document_repo.update_analysis(document_id, analysis)
        return f"Análisis guardado para documento {document_id}"

    def analyze_text(self, text: str, user_id: str) -> AnalysisResponse:
        """
        Método principal que orquesta al agente.
        """
        try:
            # 1. Definir las tools (funciones disponibles para el agente)
            tools = [
                self.search_documents,
                self.save_analysis
            ]

            # 2. Crear request
            request = AnalysisRequest(text=text, context={"user_id": user_id})

            # 3. Obtener agente desde el factory con las tools
            agent = self.agent_factory.get_agent("analysis_agent", tools=tools)

            # 4. Ejecutar agente
            response = agent.run(request)
            
            return response.content
        except Exception as e:
            raise ResponseException(500, f"Error en AnalysisService: {str(e)}")
```

#### Dependency Injection en `app/dependencies.py`

```python
from fastapi import Depends
from pymongo.database import Database
from database.database import get_db
from agents.agent_factory import AgentFactory
from services.analysis_service import AnalysisService

def get_agent_factory() -> AgentFactory:
    """Factory para crear agents (stateless)."""
    return AgentFactory()

def get_analysis_service(
    db: Database = Depends(get_db),
    agent_factory: AgentFactory = Depends(get_agent_factory)
) -> AnalysisService:
    """Inyecta db y agent_factory al servicio."""
    return AnalysisService(db, agent_factory)
```

#### Router usando el Servicio

```python
from fastapi import APIRouter, Depends, Body
from dependencies import get_analysis_service
from services.analysis_service import AnalysisService
from models.agent_model import AnalysisResponse

router = APIRouter(prefix="/analysis", tags=["Analysis"])

@router.post("/analyze", response_model=AnalysisResponse)
def analyze_text(
    text: str = Body(..., embed=True),
    user_id: str = Body(...),
    service: AnalysisService = Depends(get_analysis_service)
):
    return service.analyze_text(text, user_id)
```

## Flujo de Datos

```
Usuario
  ↓
Router → Service (con repos y tools)
           ↓
         Factory.get_agent("agent_name", tools=[...])
           ↓
         Agent.build(tools=kwargs.get("tools"))
           ↓
         Agno Agent (con tools inyectadas)
           ↓
         Ejecuta LLM con tools disponibles
           ↓
         ResponseSchema
           ↓
         Service retorna resultado
           ↓
         Router responde al usuario
```

## Patrones Importantes

### 1. Tools en Servicios vs Agente

**❌ NO hagas esto en el agente:**
```python
# INCORRECTO
class MyAgent(BaseAgent):
    def build(self, *args, **kwargs):
        return Agent(
            # ...
            tools=[self.some_tool]  # Definir tools aquí
        )
```

**✅ SÍ haz esto:**
```python
# En el agente:
class MyAgent(BaseAgent):
    def build(self, *args, **kwargs):
        return Agent(
            # ...
            tools=kwargs.get("tools", [])  # Recibir tools desde kwargs
        )

# En el servicio:
class MyService:
    def process(self):
        tools = [self.tool1, self.tool2]  # Definir aquí
        agent = self.factory.get_agent("my_agent", tools=tools)
```

### 2. BaseAgent y Persistencia

El `BaseAgent` proporciona el método `_get_db()`:

```python
class BaseAgent(ABC):
    def _get_db(self, session_collection: str) -> MongoDb:
        """Crea conexión MongoDB para sesiones del agente."""
        return MongoDb(
            db_url=os.getenv("MONGODB_URI", "mongodb://localhost:27017"),
            db_name=os.getenv("DB_NAME", "dbname"),
            session_collection=session_collection,
        )
```

### 3. Docstrings para Tools

Las tools deben tener docstrings claros porque el LLM los usa para entender qué hace cada función:

```python
def get_chart_of_accounts(self, company_id: str) -> List[Dict[str, Any]]:
    """
    Obtiene el plan de cuentas completo de una empresa.
    Úsalo para conocer los códigos y nombres de las cuentas disponibles.
    """
    accounts = self.account_repo.get_by_company(company_id)
    return [account.model_dump() for account in accounts]
```

## Checklist de Validación

- [ ] Modelos Pydantic creados en `app/models/` (Request y Response)
- [ ] Agente creado en `app/agents/agents/{name}_agent.py` heredando de `BaseAgent`
- [ ] Método `build()` usa `tools=kwargs.get("tools", [])` para recibir tools dinámicamente
- [ ] Agente registrado en `AgentFactory`
- [ ] Servicio creado en `app/services/` que inyecte `AgentFactory` y defina las tools
- [ ] Dependency agregada en `app/dependencies.py`
- [ ] Docstrings claros en todas las tools del servicio
- [ ] Manejo de errores con `ResponseException`

## Ejemplo de Instrucción

> "Crea un agente llamado 'DocumentAnalyzer' que analice documentos legales y extraiga cláusulas importantes. El agente debe poder buscar documentos relacionados y guardar el análisis. Usa el skill agno-agent-creator."

Esto generaría:
1. `app/models/document_analyzer_model.py` con `DocumentAnalysisRequest` y `DocumentAnalysisResponse`
2. `app/agents/agents/document_analyzer_agent.py` con instrucciones especializadas
3. Registro en `AgentFactory`
4. Servicio `app/services/document_analyzer_service.py` con tools para búsqueda y guardado
