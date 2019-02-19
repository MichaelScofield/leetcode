import java.util.*;

/*
// Employee info
class Employee {
    // It's the unique id of each node;
    // unique id of this employee
    public int id;
    // the importance value of this employee
    public int importance;
    // the id of direct subordinates
    public List<Integer> subordinates;
};
*/
class Solution {
    public int getImportance(List<Employee> employees, int id) {
        if (employees == null || employees.isEmpty()) {
            return 0;
        }

        Map<Integer, Employee> maps = new HashMap<Integer, Employee>();
        for (Employee employee : employees) {
            maps.put(employee.id, employee);
        }
        Employee employee = maps.get(id);
        if (employee == null) {
            return 0;
        }

        Queue<Employee> subordinates = new LinkedList<Employee>();
        subordinates.add(employee);

        int total = 0;
        while ((employee = subordinates.poll()) != null) {
            total += employee.importance;
            if (employee.subordinates != null) {
                for (Integer subordinateId : employee.subordinates) {
                    Employee subordinate = maps.get(subordinateId);
                    if (subordinate != null) {
                        subordinates.add(subordinate);
                    }
                }
            }
        }
        return total;
    }
}
