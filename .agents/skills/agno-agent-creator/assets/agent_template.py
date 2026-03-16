import os
from agno.agent import Agent
from agno.models.openai import OpenAIChat  # Cambiar según el modelo necesario (DeepSeek, OpenAI, etc.)
from textwrap import dedent

from agents.agents.base_agent import BaseAgent
# Importar modelos requeridos desde models.agent_model
# from models.agent_model import {{RequestModel}}, {{ResponseModel}}

class {{ClassName}}(BaseAgent):
    """
    Agente generado automáticamente usando Agno.
    Estructura: BaseAgent, Inputs/Outputs Pydantic, Sesiones en MongoDB.
    
    IMPORTANTE: Las tools se pasan dinámicamente desde el servicio usando
    kwargs.get("tools", []). No definas tools directamente en este archivo.
    """
    
    name = "{{AgentName}}"
    model = OpenAIChat(id=os.getenv("OPENAI_MODEL", "gpt-4o"))

    def build(self, *args, **kwargs):
        """
        Construye y retorna la instancia del Agent de Agno.
        
        Args:
            *args: Argumentos posicionales (generalmente no usados)
            **kwargs: Argumentos nombrados, debe incluir:
                - tools: Lista de funciones disponibles para el agente
                        (inyectadas desde el servicio)
        
        Returns:
            Agent: Instancia configurada de Agno Agent
        """
        return Agent(
            name=self.name,
            model=self.model,
            # Configuración de persistencia de sesiones en MongoDB
            # Usa _get_db() de BaseAgent para crear la conexión
            db=self._get_db("{{collection_name}}_sessions"),
            
            description="{{Description}}",
            
            instructions=dedent("""
                # ROLE
                {{RoleDescription}}

                # CONTEXTO
                Describe el contexto en el que opera este agente.

                # INSTRUCCIONES
                1. Paso 1 del flujo de trabajo
                2. Paso 2 del flujo de trabajo
                3. Paso 3 del flujo de trabajo

                # HERRAMIENTAS DISPONIBLES
                - Lista las tools que el servicio inyectará
                - Explica cuándo usar cada una

                # REGLAS
                - Idioma: Español.
                - Tono: Profesional.
                - Otras reglas específicas del dominio
                
                # FORMATO DE SALIDA
                El agente debe retornar un objeto que cumpla con {{ResponseModel}}.
            """),
            
            # Tools inyectadas dinámicamente desde el servicio
            # NO agregues tools aquí directamente
            tools=kwargs.get("tools", []),
            
            # Esquemas Pydantic para validación de I/O
            output_schema={{ResponseModel}},
            input_schema={{RequestModel}},
            
            # Configuración de memoria/contexto
            add_history_to_context=True,
            num_history_runs=5,
            markdown=True
        )
