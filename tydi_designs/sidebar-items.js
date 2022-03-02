initSidebarItems({"fn":[["component_loop","Returns a design with one package with two components. The “top” component instantiates the “a” component twice, and connects an output “o” to an input “i” for both instances."],["duplicated_components","Returns a design with two packages with one component each, where both of these components are exactly the same. In this stage of the project, these components will be primitive, e.g. they will not have an implementation, which is typically left to a user after template generation for the component. However, Salsa deduplicates stuff in the database, so we need to make sure we can still discern between component a.x and b.x, even though x is seemingly identical and Salsa deduplicates both x-es. We currently prevent deduplication by inserting some metadata that is not really needed by back-ends."],["types","Return a design with various type configurations."]]});